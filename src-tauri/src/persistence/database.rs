use rusqlite::{Connection, Result};

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA journal_mode = WAL;")?;

    let user_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

    if user_version < 1 {
        migrate_v1(conn)?;
        conn.pragma_update(None, "user_version", 1)?;
    }

    let user_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if user_version < 2 {
        migrate_v2(conn)?;
        conn.pragma_update(None, "user_version", 2)?;
    }

    Ok(())
}

fn migrate_v1(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS save_games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slot_name TEXT UNIQUE NOT NULL,
            world_state TEXT NOT NULL,
            player_name TEXT,
            player_location TEXT,
            player_health INTEGER,
            turns_elapsed INTEGER,
            quests_completed INTEGER,
            saved_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS playthroughs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at TEXT NOT NULL,
            ended_at TEXT,
            ending_type TEXT,
            turns_taken INTEGER,
            quests_completed INTEGER,
            enemies_defeated INTEGER
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )?;
    Ok(())
}

fn migrate_v2(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS game_stats (
            key TEXT PRIMARY KEY,
            value_int INTEGER NOT NULL DEFAULT 0
        );

        INSERT OR IGNORE INTO game_stats (key, value_int) VALUES
            ('rooms_explored', 0),
            ('enemies_defeated', 0),
            ('items_collected', 0),
            ('quests_completed', 0),
            ('commands_entered', 0),
            ('deaths', 0),
            ('games_started', 0),
            ('total_turns', 0);

        CREATE TABLE IF NOT EXISTS narration_ratings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prompt_hash TEXT NOT NULL,
            rating INTEGER NOT NULL,
            model TEXT,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS custom_themes (
            name TEXT PRIMARY KEY,
            config TEXT NOT NULL
        );",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_creates_tables() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_database(&conn).unwrap();

        // Check tables exist
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='save_games'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn initialize_is_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_database(&conn).unwrap();
        initialize_database(&conn).unwrap(); // Should not error
    }
}
