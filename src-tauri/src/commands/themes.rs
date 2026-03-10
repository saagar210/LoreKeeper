use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::persistence::state::DbState;
use crate::persistence::validators::{validate_theme_config, validate_theme_name};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomThemeInfo {
    pub name: String,
    pub config: String,
}

fn sanitize_theme_info(name: String, config: String) -> Option<CustomThemeInfo> {
    let name = validate_theme_name(&name).ok()?;
    let sanitized = validate_theme_config(&config).ok()?;
    let config = serde_json::to_string(&sanitized).ok()?;
    Some(CustomThemeInfo { name, config })
}

fn save_custom_theme_record(conn: &Connection, name: &str, config: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO custom_themes (name, config) VALUES (?1, ?2)
         ON CONFLICT(name) DO UPDATE SET config = ?2",
        rusqlite::params![name, config],
    )
    .map_err(|e| format!("Failed to save theme: {}", e))?;

    Ok(())
}

fn list_custom_theme_records(conn: &Connection) -> Result<Vec<CustomThemeInfo>, String> {
    let mut stmt = conn
        .prepare("SELECT name, config FROM custom_themes ORDER BY name")
        .map_err(|e| format!("Query error: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("Query error: {}", e))?;

    let mut themes = Vec::new();
    for row in rows {
        let Ok((name, config)) = row else {
            continue;
        };
        if let Some(info) = sanitize_theme_info(name, config) {
            themes.push(info);
        }
    }

    Ok(themes)
}

#[tauri::command]
pub fn save_custom_theme(
    name: String,
    config: String,
    db_state: State<DbState>,
) -> Result<(), String> {
    let name = validate_theme_name(&name)?;
    let config = serde_json::to_string(&validate_theme_config(&config)?)
        .map_err(|e| format!("Failed to serialize theme config: {}", e))?;
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    save_custom_theme_record(&conn, &name, &config)
}

#[tauri::command]
pub fn list_custom_themes(db_state: State<DbState>) -> Result<Vec<CustomThemeInfo>, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    list_custom_theme_records(&conn)
}

#[tauri::command]
pub fn delete_custom_theme(name: String, db_state: State<DbState>) -> Result<(), String> {
    let name = validate_theme_name(&name)?;
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
    use super::{list_custom_theme_records, sanitize_theme_info, save_custom_theme_record};
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
        let config = serde_json::json!({
            "--bg": "#0A0A0A",
            "--text": "#33FF33",
            "--text-dim": "#1A8C1A",
            "--text-bright": "#66FF66",
            "--accent": "#00CC00",
            "--error": "#FF4444",
            "--combat": "#FF6666",
            "--dialogue": "#66CCCC",
            "--input": "#FFAA33",
            "--system": "#888888",
            "--border": "#1A3A1A",
            "--panel-bg": "#0D0D0D",
            "--hp-high": "#33FF33",
            "--hp-mid": "#FFCC00",
            "--hp-low": "#FF4444"
        })
        .to_string();

        save_custom_theme_record(&conn, "test", &config).unwrap();

        let themes = list_custom_theme_records(&conn).unwrap();
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].name, "test");
        assert!(themes[0].config.contains("\"--bg\":\"#0A0A0A\""));
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

    #[test]
    fn list_custom_themes_skips_invalid_legacy_rows() {
        let conn = setup_db();
        let valid_config = serde_json::json!({
            "--bg": "#0A0A0A",
            "--text": "#33FF33",
            "--text-dim": "#1A8C1A",
            "--text-bright": "#66FF66",
            "--accent": "#00CC00",
            "--error": "#FF4444",
            "--combat": "#FF6666",
            "--dialogue": "#66CCCC",
            "--input": "#FFAA33",
            "--system": "#888888",
            "--border": "#1A3A1A",
            "--panel-bg": "#0D0D0D",
            "--hp-high": "#33FF33",
            "--hp-mid": "#FFCC00",
            "--hp-low": "#FF4444"
        })
        .to_string();
        conn.execute(
            "INSERT INTO custom_themes (name, config) VALUES (?1, ?2), (?3, ?4)",
            rusqlite::params![
                "good_theme",
                valid_config,
                "bad/theme",
                "{\"--bg\":\"nope\"}"
            ],
        )
        .unwrap();

        let themes = list_custom_theme_records(&conn).unwrap();
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].name, "good_theme");
    }

    #[test]
    fn sanitize_theme_info_rejects_invalid_name_or_config() {
        assert!(sanitize_theme_info("bad/theme".into(), "{}".into()).is_none());
        assert!(sanitize_theme_info("good_theme".into(), "{\"--bg\":\"red\"}".into()).is_none());
    }
}
