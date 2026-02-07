use tauri::State;

use crate::models::CommandResponse;
use crate::persistence::save_load::{self, SaveSlotInfo};
use crate::persistence::state::{DbState, GameState};

#[tauri::command]
pub fn save_game(
    slot_name: String,
    game_state: State<GameState>,
    db_state: State<DbState>,
) -> Result<(), String> {
    let state = game_state.0.lock().map_err(|e| e.to_string())?;
    let db = db_state.0.lock().map_err(|e| e.to_string())?;
    save_load::save_game(&db, &slot_name, &state)
}

#[tauri::command]
pub fn load_game(
    slot_name: String,
    game_state: State<GameState>,
    db_state: State<DbState>,
) -> Result<CommandResponse, String> {
    let mut state = game_state.0.lock().map_err(|e| e.to_string())?;
    let db = db_state.0.lock().map_err(|e| e.to_string())?;
    let loaded = save_load::load_game(&db, &slot_name)?;
    *state = loaded;

    let loc = state.locations.get(&state.player.location).cloned();
    let mut messages = vec![crate::models::OutputLine {
        text: format!("Game loaded from '{}'.", slot_name),
        line_type: crate::models::LineType::System,
    }];
    if let Some(location) = loc {
        let look_lines = crate::engine::templates::describe_location(
            &location,
            &state.items,
            &state.npcs,
            false,
        );
        messages.extend(look_lines.into_iter().map(|text| crate::models::OutputLine {
            text,
            line_type: crate::models::LineType::Narration,
        }));
    }

    Ok(CommandResponse {
        messages,
        world_state: state.clone(),
    })
}

#[tauri::command]
pub fn list_saves(db_state: State<DbState>) -> Result<Vec<SaveSlotInfo>, String> {
    let db = db_state.0.lock().map_err(|e| e.to_string())?;
    save_load::list_saves(&db)
}

#[tauri::command]
pub fn delete_save(slot_name: String, db_state: State<DbState>) -> Result<(), String> {
    let db = db_state.0.lock().map_err(|e| e.to_string())?;
    save_load::delete_save(&db, &slot_name)
}
