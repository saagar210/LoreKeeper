use tauri::State;

use crate::models::GameSettings;
use crate::narrative::ollama::{ModelInfo, OllamaClient, OllamaStatus};
use crate::persistence::save_load;
use crate::persistence::state::{DbState, SettingsState};

#[tauri::command]
pub fn get_settings(settings_state: State<SettingsState>) -> Result<GameSettings, String> {
    let settings = settings_state.0.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
pub fn update_settings(
    settings: GameSettings,
    db_state: State<DbState>,
    settings_state: State<SettingsState>,
) -> Result<(), String> {
    // Lock ordering: always SettingsState before DbState to match game.rs
    let mut current = settings_state.0.lock().map_err(|e| e.to_string())?;
    let db = db_state.0.lock().map_err(|e| e.to_string())?;
    save_load::save_settings(&db, &settings)?;
    drop(db);

    *current = settings;
    Ok(())
}

#[tauri::command]
pub async fn get_ollama_status(
    settings_state: State<'_, SettingsState>,
) -> Result<OllamaStatus, String> {
    let url = {
        let settings = settings_state.0.lock().map_err(|e| e.to_string())?;
        settings.ollama_url.clone()
    };
    let client = OllamaClient::new(&url);
    client.check_health().await
}

#[tauri::command]
pub async fn get_available_models(
    settings_state: State<'_, SettingsState>,
) -> Result<Vec<ModelInfo>, String> {
    let url = {
        let settings = settings_state.0.lock().map_err(|e| e.to_string())?;
        settings.ollama_url.clone()
    };
    let client = OllamaClient::new(&url);
    client.list_models().await
}
