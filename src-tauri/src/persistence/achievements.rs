use rusqlite::{params, Connection};

use crate::models::achievement::{Achievement, AchievementInfo, all_achievements};

pub fn unlock_achievement(conn: &Connection, id: &str) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR IGNORE INTO achievements (id, unlocked_at) VALUES (?1, ?2)",
        params![id, now],
    )
    .map_err(|e| format!("Achievement unlock error: {}", e))?;
    Ok(())
}

pub fn is_unlocked(conn: &Connection, id: &str) -> bool {
    conn.query_row(
        "SELECT COUNT(*) FROM achievements WHERE id = ?1",
        params![id],
        |row| row.get::<_, i32>(0),
    )
    .map(|count| count > 0)
    .unwrap_or(false)
}

pub fn get_unlocked_achievements(conn: &Connection) -> Result<Vec<AchievementInfo>, String> {
    let all = all_achievements();
    let mut result = Vec::with_capacity(all.len());

    let mut stmt = conn
        .prepare("SELECT id, unlocked_at FROM achievements")
        .map_err(|e| format!("Achievement query error: {}", e))?;

    let unlocked: std::collections::HashMap<String, String> = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("Achievement query error: {}", e))?
        .collect::<Result<std::collections::HashMap<_, _>, _>>()
        .map_err(|e| format!("Achievement row error: {}", e))?;

    for Achievement { id, name, description, icon } in all {
        let (is_unlocked, unlocked_at) = match unlocked.get(&id) {
            Some(at) => (true, Some(at.clone())),
            None => (false, None),
        };
        result.push(AchievementInfo {
            id,
            name,
            description,
            icon,
            unlocked: is_unlocked,
            unlocked_at,
        });
    }

    Ok(result)
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
    fn unlock_and_check() {
        let conn = setup_db();
        assert!(!is_unlocked(&conn, "first_blood"));
        unlock_achievement(&conn, "first_blood").unwrap();
        assert!(is_unlocked(&conn, "first_blood"));
    }

    #[test]
    fn unlock_is_idempotent() {
        let conn = setup_db();
        unlock_achievement(&conn, "explorer").unwrap();
        unlock_achievement(&conn, "explorer").unwrap(); // Should not error
        assert!(is_unlocked(&conn, "explorer"));
    }

    #[test]
    fn get_all_shows_unlocked_status() {
        let conn = setup_db();
        unlock_achievement(&conn, "first_blood").unwrap();
        let achievements = get_unlocked_achievements(&conn).unwrap();
        assert_eq!(achievements.len(), all_achievements().len());
        let fb = achievements.iter().find(|a| a.id == "first_blood").unwrap();
        assert!(fb.unlocked);
        assert!(fb.unlocked_at.is_some());
        let explorer = achievements.iter().find(|a| a.id == "explorer").unwrap();
        assert!(!explorer.unlocked);
        assert!(explorer.unlocked_at.is_none());
    }
}
