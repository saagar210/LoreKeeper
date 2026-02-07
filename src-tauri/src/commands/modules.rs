use tauri::{Manager, State};

use crate::engine::module_loader;
use crate::models::module::ModuleInfo;
use crate::models::{CommandResponse, LineType, OutputLine, WorldState};
use crate::persistence::state::GameState;

#[tauri::command]
pub fn list_modules(app: tauri::AppHandle) -> Result<Vec<ModuleInfo>, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let modules_dir = app_data_dir.join("modules");

    if !modules_dir.exists() {
        std::fs::create_dir_all(&modules_dir).ok();
        return Ok(vec![]);
    }

    let mut modules = Vec::new();
    let entries = std::fs::read_dir(&modules_dir).map_err(|e| format!("Read dir error: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str::<WorldState>(&content) {
                    let name = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();
                    modules.push(ModuleInfo {
                        name,
                        description: format!(
                            "{} locations, {} items",
                            state.locations.len(),
                            state.items.len()
                        ),
                        path: path.to_string_lossy().to_string(),
                        location_count: state.locations.len(),
                        item_count: state.items.len(),
                    });
                }
            }
        }
    }

    Ok(modules)
}

#[tauri::command]
pub fn load_module(
    path: String,
    app: tauri::AppHandle,
    game_state: State<GameState>,
) -> Result<CommandResponse, String> {
    // Validate that the path is within the app's modules directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let modules_dir = app_data_dir.join("modules");
    let requested = std::path::Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    if !requested.starts_with(&modules_dir) {
        return Err("Module path must be within the modules directory.".into());
    }

    let loaded = module_loader::load_module(&requested)?;

    let loc = loaded.locations.get(&loaded.player.location).cloned();
    let mut state = game_state.0.lock().map_err(|e| e.to_string())?;
    *state = loaded;

    let messages = if let Some(location) = loc {
        let mut msgs = vec![
            OutputLine {
                text: "Module loaded. A new adventure begins...".into(),
                line_type: LineType::System,
            },
            OutputLine {
                text: String::new(),
                line_type: LineType::System,
            },
        ];
        let look_lines = crate::engine::templates::describe_location(
            &location,
            &state.items,
            &state.npcs,
            true,
        );
        msgs.extend(look_lines.into_iter().map(|text| OutputLine {
            text,
            line_type: LineType::Narration,
        }));
        msgs
    } else {
        vec![OutputLine {
            text: "Module loaded but starting location not found.".into(),
            line_type: LineType::Error,
        }]
    };

    Ok(CommandResponse {
        messages,
        world_state: state.clone(),
    })
}
