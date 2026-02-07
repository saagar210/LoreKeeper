use rusqlite::{params, Connection};
use std::collections::HashMap;

pub fn increment_stat(conn: &Connection, key: &str, amount: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE game_stats SET value_int = value_int + ?2 WHERE key = ?1",
        params![key, amount],
    )
    .map_err(|e| format!("Stats error: {}", e))?;
    Ok(())
}

pub fn get_all_stats(conn: &Connection) -> Result<HashMap<String, i64>, String> {
    let mut stmt = conn
        .prepare("SELECT key, value_int FROM game_stats")
        .map_err(|e| format!("Stats query error: {}", e))?;

    let map = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })
        .map_err(|e| format!("Stats query error: {}", e))?
        .collect::<Result<HashMap<String, i64>, _>>()
        .map_err(|e| format!("Stats row error: {}", e))?;

    Ok(map)
}

pub fn reset_stats(conn: &Connection) -> Result<(), String> {
    conn.execute("UPDATE game_stats SET value_int = 0", [])
        .map_err(|e| format!("Stats reset error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::database;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        database::initialize_database(&conn).unwrap();
        conn
    }

    #[test]
    fn increment_and_read() {
        let conn = setup_db();
        increment_stat(&conn, "rooms_explored", 3).unwrap();
        let stats = get_all_stats(&conn).unwrap();
        assert_eq!(*stats.get("rooms_explored").unwrap(), 3);
    }

    #[test]
    fn reset_clears_all() {
        let conn = setup_db();
        increment_stat(&conn, "deaths", 5).unwrap();
        reset_stats(&conn).unwrap();
        let stats = get_all_stats(&conn).unwrap();
        assert_eq!(*stats.get("deaths").unwrap(), 0);
    }

    #[test]
    fn all_stat_keys_present() {
        let conn = setup_db();
        let stats = get_all_stats(&conn).unwrap();
        assert!(stats.contains_key("rooms_explored"));
        assert!(stats.contains_key("enemies_defeated"));
        assert!(stats.contains_key("items_collected"));
        assert!(stats.contains_key("quests_completed"));
        assert!(stats.contains_key("commands_entered"));
        assert!(stats.contains_key("deaths"));
        assert!(stats.contains_key("games_started"));
        assert!(stats.contains_key("total_turns"));
    }
}
