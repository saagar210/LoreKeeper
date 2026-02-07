use tauri::State;

use crate::models::CombatLogEntry;
use crate::persistence::state::GameState;

#[tauri::command]
pub fn get_combat_log(game_state: State<GameState>) -> Result<Vec<CombatLogEntry>, String> {
    let state = game_state.0.lock().map_err(|e| e.to_string())?;
    Ok(state.combat_log.clone())
}
