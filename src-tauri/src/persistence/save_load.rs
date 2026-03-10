use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::models::WorldState;
use crate::persistence::validators::{validate_existing_save_slot_name, validate_save_slot_name};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSlotInfo {
    pub slot_name: String,
    pub player_location: Option<String>,
    pub player_health: Option<i32>,
    pub turns_elapsed: Option<i32>,
    pub quests_completed: Option<i32>,
    pub saved_at: String,
}

pub fn save_game(conn: &Connection, slot_name: &str, state: &WorldState) -> Result<(), String> {
    let slot_name = resolve_save_slot_name_for_write(conn, slot_name)?;
    let json = serde_json::to_string(state).map_err(|e| format!("Serialize error: {}", e))?;

    let location = state
        .locations
        .get(&state.player.location)
        .map(|l| l.name.clone())
        .unwrap_or_else(|| state.player.location.clone());

    let quests_completed = state.quests.values().filter(|q| q.completed).count() as i32;

    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(slot_name) DO UPDATE SET
            world_state = ?2,
            player_location = ?3,
            player_health = ?4,
            turns_elapsed = ?5,
            quests_completed = ?6,
            saved_at = ?7",
        params![
            slot_name,
            json,
            location,
            state.player.health,
            state.player.turns_elapsed as i32,
            quests_completed,
            now
        ],
    )
    .map_err(|e| format!("Save error: {}", e))?;

    Ok(())
}

fn resolve_save_slot_name_for_write(conn: &Connection, slot_name: &str) -> Result<String, String> {
    if let Ok(normalized) = validate_save_slot_name(slot_name) {
        return Ok(normalized);
    }

    let legacy_name = validate_existing_save_slot_name(slot_name)?;
    let exists = conn
        .query_row(
            "SELECT 1 FROM save_games WHERE slot_name = ?1 LIMIT 1",
            params![legacy_name],
            |_| Ok(()),
        )
        .is_ok();

    if exists {
        return Ok(slot_name.to_string());
    }

    validate_save_slot_name(slot_name)
}

pub fn load_game(conn: &Connection, slot_name: &str) -> Result<WorldState, String> {
    let slot_name = validate_existing_save_slot_name(slot_name)?;
    let json: String = conn
        .query_row(
            "SELECT world_state FROM save_games WHERE slot_name = ?1",
            params![slot_name],
            |row| row.get(0),
        )
        .map_err(|e| format!("Save not found: {}", e))?;

    serde_json::from_str(&json)
        .map_err(|_| "Save data is corrupted and could not be loaded.".to_string())
}

pub fn list_saves(conn: &Connection) -> Result<Vec<SaveSlotInfo>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT slot_name, player_location, player_health, turns_elapsed, quests_completed, saved_at
             FROM save_games ORDER BY saved_at DESC",
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(SaveSlotInfo {
                slot_name: row.get(0)?,
                player_location: row.get(1)?,
                player_health: row.get(2)?,
                turns_elapsed: row.get(3)?,
                quests_completed: row.get(4)?,
                saved_at: row.get(5)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?;

    let mut saves = Vec::new();
    for row in rows {
        let Ok(info) = row else {
            continue;
        };
        if let Some(info) = sanitize_save_slot_info(info) {
            saves.push(info);
        }
    }

    Ok(saves)
}

pub fn delete_save(conn: &Connection, slot_name: &str) -> Result<(), String> {
    let slot_name = validate_existing_save_slot_name(slot_name)?;
    conn.execute(
        "DELETE FROM save_games WHERE slot_name = ?1",
        params![slot_name],
    )
    .map_err(|e| format!("Delete error: {}", e))?;
    Ok(())
}

pub fn save_settings(
    conn: &Connection,
    settings: &crate::models::GameSettings,
) -> Result<(), String> {
    let json = serde_json::to_string(settings).map_err(|e| format!("Serialize error: {}", e))?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('game_settings', ?1)
         ON CONFLICT(key) DO UPDATE SET value = ?1",
        params![json],
    )
    .map_err(|e| format!("Save settings error: {}", e))?;
    Ok(())
}

pub fn load_settings(conn: &Connection) -> Option<crate::models::GameSettings> {
    let result: Result<String, _> = conn.query_row(
        "SELECT value FROM settings WHERE key = 'game_settings'",
        [],
        |row| row.get(0),
    );
    match result {
        Ok(json) => serde_json::from_str(&json)
            .ok()
            .map(crate::models::GameSettings::sanitize_loaded),
        Err(_) => None,
    }
}

fn sanitize_save_slot_info(mut info: SaveSlotInfo) -> Option<SaveSlotInfo> {
    let slot_name = validate_existing_save_slot_name(&info.slot_name).ok()?;
    if info.saved_at.trim().is_empty() {
        return None;
    }
    info.slot_name = slot_name;
    Some(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;
    use crate::persistence::database;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        database::initialize_database(&conn).unwrap();
        conn
    }

    #[test]
    fn save_and_load_roundtrip() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();

        save_game(&conn, "test_slot", &state).unwrap();
        let loaded = load_game(&conn, "test_slot").unwrap();

        assert_eq!(loaded.player.location, state.player.location);
        assert_eq!(loaded.player.health, state.player.health);
        assert_eq!(loaded.locations.len(), state.locations.len());
        assert_eq!(loaded.items.len(), state.items.len());
    }

    #[test]
    fn list_saves_returns_saved_games() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();

        save_game(&conn, "slot_1", &state).unwrap();
        save_game(&conn, "slot_2", &state).unwrap();

        let saves = list_saves(&conn).unwrap();
        assert_eq!(saves.len(), 2);
    }

    #[test]
    fn list_saves_skips_invalid_rows() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
             VALUES (?1, '{}', 'Courtyard', 100, 1, 0, '2025-01-01T00:00:00Z')",
            params!["bad\nsave"],
        )
        .unwrap();

        let saves = list_saves(&conn).unwrap();
        assert!(saves.is_empty());
    }

    #[test]
    fn list_saves_keeps_legacy_names_accessible() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();
        let json = serde_json::to_string(&state).unwrap();
        conn.execute(
            "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
             VALUES ('legacy/save:1', ?1, 'Courtyard', 100, 1, 0, '2025-01-01T00:00:00Z')",
            params![json],
        )
        .unwrap();

        let saves = list_saves(&conn).unwrap();
        assert_eq!(saves.len(), 1);
        assert_eq!(saves[0].slot_name, "legacy/save:1");
    }

    #[test]
    fn delete_save_removes_slot() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();

        save_game(&conn, "to_delete", &state).unwrap();
        assert!(load_game(&conn, "to_delete").is_ok());

        delete_save(&conn, "to_delete").unwrap();
        assert!(load_game(&conn, "to_delete").is_err());
    }

    #[test]
    fn save_overwrites_existing_slot() {
        let conn = setup_db();
        let mut state = world_builder::build_thornhold();

        save_game(&conn, "overwrite", &state).unwrap();
        state.player.health = 50;
        save_game(&conn, "overwrite", &state).unwrap();

        let loaded = load_game(&conn, "overwrite").unwrap();
        assert_eq!(loaded.player.health, 50);
    }

    #[test]
    fn save_slot_names_are_validated() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();

        let result = save_game(&conn, "bad/save", &state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Save name"));
    }

    #[test]
    fn save_game_allows_overwriting_legacy_slot_names() {
        let conn = setup_db();
        let mut state = world_builder::build_thornhold();
        let json = serde_json::to_string(&state).unwrap();
        conn.execute(
            "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
             VALUES ('legacy/save:1', ?1, 'Courtyard', 100, 1, 0, '2025-01-01T00:00:00Z')",
            params![json],
        )
        .unwrap();

        state.player.health = 42;
        save_game(&conn, "legacy/save:1", &state).unwrap();

        let loaded = load_game(&conn, "legacy/save:1").unwrap();
        assert_eq!(loaded.player.health, 42);
    }

    #[test]
    fn load_and_delete_reject_invalid_slot_names() {
        let conn = setup_db();

        let load_result = load_game(&conn, "bad\nsave");
        assert!(load_result.is_err());
        assert!(load_result.unwrap_err().contains("Save name"));

        let delete_result = delete_save(&conn, "bad\nsave");
        assert!(delete_result.is_err());
        assert!(delete_result.unwrap_err().contains("Save name"));
    }

    #[test]
    fn load_and_delete_allow_legacy_slot_names() {
        let conn = setup_db();
        let state = world_builder::build_thornhold();
        let json = serde_json::to_string(&state).unwrap();
        conn.execute(
            "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
             VALUES ('legacy/save:1', ?1, 'Courtyard', 100, 1, 0, '2025-01-01T00:00:00Z')",
            params![json],
        )
        .unwrap();

        let loaded = load_game(&conn, "legacy/save:1").unwrap();
        assert_eq!(loaded.player.location, state.player.location);

        delete_save(&conn, "legacy/save:1").unwrap();
        assert!(list_saves(&conn).unwrap().is_empty());
    }

    #[test]
    fn load_game_returns_corruption_message_for_bad_json() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO save_games (slot_name, world_state, player_location, player_health, turns_elapsed, quests_completed, saved_at)
             VALUES ('valid_save', 'not-json', 'Courtyard', 100, 1, 0, '2025-01-01T00:00:00Z')",
            [],
        )
        .unwrap();

        let result = load_game(&conn, "valid_save");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Save data is corrupted and could not be loaded."));
    }

    #[test]
    fn settings_save_and_load() {
        let conn = setup_db();
        let settings = crate::models::GameSettings::default();
        save_settings(&conn, &settings).unwrap();
        let loaded = load_settings(&conn).unwrap();
        assert_eq!(loaded.ollama_model, settings.ollama_model);
    }

    #[test]
    fn load_settings_sanitizes_invalid_ollama_url() {
        let conn = setup_db();
        let settings = crate::models::GameSettings {
            ollama_enabled: true,
            ollama_url: "http://example.com:11434".into(),
            ..crate::models::GameSettings::default()
        };

        save_settings(&conn, &settings).unwrap();
        let loaded = load_settings(&conn).unwrap();

        assert!(!loaded.ollama_enabled);
        assert_eq!(loaded.ollama_url, "http://localhost:11434");
    }

    #[test]
    fn load_settings_returns_none_for_malformed_json() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('game_settings', 'not-json')
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [],
        )
        .unwrap();

        assert!(load_settings(&conn).is_none());
    }
}
