use tauri::{Manager, State};

use crate::engine::module_loader;
use crate::models::module::ModuleInfo;
use crate::models::{CommandResponse, LineType, OutputLine, WorldState};
use crate::persistence::state::GameState;

fn validate_module_id(module_id: &str) -> Result<(), String> {
    let trimmed = module_id.trim();
    if trimmed.is_empty() {
        return Err("Module id cannot be empty.".into());
    }
    if trimmed.starts_with('.') {
        return Err("Module id cannot start with '.'.".into());
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains(':') {
        return Err("Module id must be a file name, not a path.".into());
    }
    if !trimmed.ends_with(".json") {
        return Err("Module id must end with '.json'.".into());
    }
    Ok(())
}

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
                    let Some(module_id) = path.file_name().map(|s| s.to_string_lossy().to_string()) else {
                        continue;
                    };
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
                        module_id,
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
    module_id: String,
    app: tauri::AppHandle,
    game_state: State<GameState>,
) -> Result<CommandResponse, String> {
    validate_module_id(&module_id)?;

    // Validate that the path is within the app's modules directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let modules_dir = app_data_dir.join("modules");
    // Ensure modules_dir exists before canonicalize
    if !modules_dir.exists() {
        std::fs::create_dir_all(&modules_dir)
            .map_err(|e| format!("Failed to create modules dir: {}", e))?;
    }
    let canonical_modules_dir = modules_dir
        .canonicalize()
        .map_err(|e| format!("Failed to resolve modules dir: {}", e))?;
    let requested = modules_dir
        .join(&module_id)
        .canonicalize()
        .map_err(|e| format!("Invalid module id: {}", e))?;
    if !requested.starts_with(&canonical_modules_dir) {
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
        sound_cues: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::validate_module_id;

    #[test]
    fn accepts_json_file_name() {
        assert!(validate_module_id("thornhold.json").is_ok());
    }

    #[test]
    fn rejects_path_traversal_like_values() {
        assert!(validate_module_id("../thornhold.json").is_err());
        assert!(validate_module_id("mods/thornhold.json").is_err());
        assert!(validate_module_id("mods\\thornhold.json").is_err());
    }

    #[test]
    fn rejects_non_json_module_ids() {
        assert!(validate_module_id("thornhold").is_err());
        assert!(validate_module_id(".hidden.json").is_err());
    }
}
