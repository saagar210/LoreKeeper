use serde::{Deserialize, Serialize};
use tauri::State;

use crate::persistence::state::DbState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomThemeInfo {
    pub name: String,
    pub config: String,
}

#[tauri::command]
pub fn save_custom_theme(
    name: String,
    config: String,
    db_state: State<DbState>,
) -> Result<(), String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO custom_themes (name, config) VALUES (?1, ?2)
         ON CONFLICT(name) DO UPDATE SET config = ?2",
        rusqlite::params![name, config],
    )
    .map_err(|e| format!("Failed to save theme: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn list_custom_themes(db_state: State<DbState>) -> Result<Vec<CustomThemeInfo>, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT name, config FROM custom_themes ORDER BY name")
        .map_err(|e| format!("Query error: {}", e))?;
    let themes = stmt
        .query_map([], |row| {
            Ok(CustomThemeInfo {
                name: row.get(0)?,
                config: row.get(1)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;
    Ok(themes)
}

#[tauri::command]
pub fn delete_custom_theme(name: String, db_state: State<DbState>) -> Result<(), String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM custom_themes WHERE name = ?1",
        rusqlite::params![name],
    )
    .map_err(|e| format!("Delete error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::persistence::database;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        database::initialize_database(&conn).unwrap();
        conn
    }

    #[test]
    fn save_and_list_custom_themes() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO custom_themes (name, config) VALUES ('test', '{\"--bg\":\"#000\"}')",
            [],
        )
        .unwrap();

        let mut stmt = conn.prepare("SELECT name, config FROM custom_themes").unwrap();
        let themes: Vec<(String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].0, "test");
    }

    #[test]
    fn delete_custom_theme() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO custom_themes (name, config) VALUES ('to_delete', '{}')",
            [],
        )
        .unwrap();
        conn.execute("DELETE FROM custom_themes WHERE name = 'to_delete'", [])
            .unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM custom_themes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
