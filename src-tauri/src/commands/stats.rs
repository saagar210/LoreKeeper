use std::collections::HashMap;
use tauri::State;

use crate::persistence::state::DbState;
use crate::persistence::stats;

#[tauri::command]
pub fn get_stats(db_state: State<DbState>) -> Result<HashMap<String, i64>, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    stats::get_all_stats(&conn)
}

#[tauri::command]
pub fn reset_stats(db_state: State<DbState>) -> Result<(), String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    stats::reset_stats(&conn)
}
