use tauri::Manager;

use crate::models::achievement::AchievementInfo;
use crate::persistence::state::DbState;

#[tauri::command]
pub fn get_achievements(app: tauri::AppHandle) -> Result<Vec<AchievementInfo>, String> {
    let db_state = app.state::<DbState>();
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    crate::persistence::achievements::get_unlocked_achievements(&conn)
}
