use tauri::State;

use crate::models::CommandLogEntry;
use crate::persistence::state::DbState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayInfo {
    pub id: i64,
    pub ended_at: String,
    pub ending_type: Option<String>,
    pub turns_taken: Option<i32>,
    pub quests_completed: Option<i32>,
    pub command_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayDetail {
    pub info: ReplayInfo,
    pub commands: Vec<CommandLogEntry>,
}

#[tauri::command]
pub fn list_replays(db_state: State<DbState>) -> Result<Vec<ReplayInfo>, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, ended_at, ending_type, turns_taken, quests_completed, command_log \
             FROM playthroughs \
             WHERE command_log IS NOT NULL AND command_log != '' \
             ORDER BY id DESC LIMIT 20",
        )
        .map_err(|e| e.to_string())?;

    let replays = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let ended_at: String = row.get(1)?;
            let ending_type: Option<String> = row.get(2)?;
            let turns_taken: Option<i32> = row.get(3)?;
            let quests_completed: Option<i32> = row.get(4)?;
            let log_json: String = row.get(5)?;
            let command_count = serde_json::from_str::<Vec<serde_json::Value>>(&log_json)
                .map(|v| v.len())
                .unwrap_or(0);
            Ok(ReplayInfo {
                id,
                ended_at,
                ending_type,
                turns_taken,
                quests_completed,
                command_count,
            })
        })
        .map_err(|e| e.to_string())?;

    replays
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_replay(id: i64, db_state: State<DbState>) -> Result<ReplayDetail, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    let row = conn
        .query_row(
            "SELECT id, ended_at, ending_type, turns_taken, quests_completed, command_log \
             FROM playthroughs WHERE id = ?1",
            [id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<i32>>(3)?,
                    row.get::<_, Option<i32>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .map_err(|e| e.to_string())?;

    let commands: Vec<CommandLogEntry> = row
        .5
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    let command_count = commands.len();

    Ok(ReplayDetail {
        info: ReplayInfo {
            id: row.0,
            ended_at: row.1,
            ending_type: row.2,
            turns_taken: row.3,
            quests_completed: row.4,
            command_count,
        },
        commands,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::database;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        database::initialize_database(&conn).unwrap();
        conn
    }

    #[test]
    fn list_replays_empty_when_no_playthroughs() {
        let conn = setup_db();
        let mut stmt = conn
            .prepare(
                "SELECT id, ended_at, ending_type, turns_taken, quests_completed, command_log \
                 FROM playthroughs \
                 WHERE command_log IS NOT NULL AND command_log != '' \
                 ORDER BY id DESC LIMIT 20",
            )
            .unwrap();

        let replays: Vec<ReplayInfo> = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let ended_at: String = row.get(1)?;
                let ending_type: Option<String> = row.get(2)?;
                let turns_taken: Option<i32> = row.get(3)?;
                let quests_completed: Option<i32> = row.get(4)?;
                let log_json: String = row.get(5)?;
                let command_count = serde_json::from_str::<Vec<serde_json::Value>>(&log_json)
                    .map(|v| v.len())
                    .unwrap_or(0);
                Ok(ReplayInfo {
                    id,
                    ended_at,
                    ending_type,
                    turns_taken,
                    quests_completed,
                    command_count,
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(replays.is_empty());
    }

    #[test]
    fn get_replay_returns_error_for_nonexistent_id() {
        let conn = setup_db();
        let result = conn.query_row(
            "SELECT id FROM playthroughs WHERE id = ?1",
            [999],
            |row| row.get::<_, i64>(0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn list_replays_returns_inserted_playthrough() {
        let conn = setup_db();
        let log_json = serde_json::to_string(&vec![CommandLogEntry {
            turn: 0,
            input: "go north".to_string(),
            location: "courtyard".to_string(),
            timestamp_ms: 1700000000000,
        }])
        .unwrap();

        conn.execute(
            "INSERT INTO playthroughs (started_at, ended_at, ending_type, turns_taken, quests_completed, enemies_defeated, command_log) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                "2025-01-01T00:00:00Z",
                "2025-01-01T01:00:00Z",
                "VictoryPeace",
                42,
                3,
                5,
                log_json,
            ],
        ).unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT id, ended_at, ending_type, turns_taken, quests_completed, command_log \
                 FROM playthroughs \
                 WHERE command_log IS NOT NULL AND command_log != '' \
                 ORDER BY id DESC LIMIT 20",
            )
            .unwrap();

        let replays: Vec<ReplayInfo> = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let ended_at: String = row.get(1)?;
                let ending_type: Option<String> = row.get(2)?;
                let turns_taken: Option<i32> = row.get(3)?;
                let quests_completed: Option<i32> = row.get(4)?;
                let log_json: String = row.get(5)?;
                let command_count = serde_json::from_str::<Vec<serde_json::Value>>(&log_json)
                    .map(|v| v.len())
                    .unwrap_or(0);
                Ok(ReplayInfo {
                    id,
                    ended_at,
                    ending_type,
                    turns_taken,
                    quests_completed,
                    command_count,
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(replays.len(), 1);
        assert_eq!(replays[0].command_count, 1);
        assert_eq!(replays[0].ending_type, Some("VictoryPeace".to_string()));
        assert_eq!(replays[0].turns_taken, Some(42));
    }

    #[test]
    fn replay_info_serde_roundtrip() {
        let info = ReplayInfo {
            id: 1,
            ended_at: "2025-01-01T00:00:00Z".to_string(),
            ending_type: Some("Death".to_string()),
            turns_taken: Some(10),
            quests_completed: Some(2),
            command_count: 5,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("commandCount"));
        let deserialized: ReplayInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, 1);
        assert_eq!(deserialized.command_count, 5);
    }
}
