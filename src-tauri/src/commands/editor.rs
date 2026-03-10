use tauri::Manager;

use crate::models::WorldState;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleExportResult {
    pub module_id: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

fn sanitize_module_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Module name cannot be empty.".into());
    }
    if trimmed.len() > 64 {
        return Err("Module name must be 64 characters or fewer.".into());
    }
    if trimmed
        .chars()
        .any(|c| c.is_control() || matches!(c, '.' | '/' | '\\' | ':'))
    {
        return Err(
            "Module name cannot contain '.', '/', '\\', ':', or control characters.".into(),
        );
    }

    let slug = trimmed
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    if slug.is_empty() {
        return Err("Module name must include letters or numbers.".into());
    }

    Ok(format!("{slug}.json"))
}

#[tauri::command]
pub fn validate_module_json(json: String) -> Result<ValidationResult, String> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let state: WorldState = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            return Ok(ValidationResult {
                valid: false,
                errors: vec![format!("Invalid JSON: {}", e)],
                warnings: vec![],
            });
        }
    };

    // Must have at least one location
    if state.locations.is_empty() {
        errors.push("Module must have at least one location.".into());
    }

    // Player starting location must exist
    if !state.locations.contains_key(&state.player.location) {
        errors.push(format!(
            "Starting location '{}' does not exist.",
            state.player.location
        ));
    }

    // Validate exits point to existing locations
    for (loc_id, loc) in &state.locations {
        for (dir, dest) in &loc.exits {
            if !state.locations.contains_key(dest) {
                errors.push(format!(
                    "Location '{}' exit {:?} points to non-existent '{}'.",
                    loc_id, dir, dest
                ));
            }
        }
        // Validate items exist
        for item_id in &loc.items {
            if !state.items.contains_key(item_id) {
                errors.push(format!(
                    "Location '{}' references non-existent item '{}'.",
                    loc_id, item_id
                ));
            }
        }
        // Validate NPCs exist
        for npc_id in &loc.npcs {
            if !state.npcs.contains_key(npc_id) {
                errors.push(format!(
                    "Location '{}' references non-existent NPC '{}'.",
                    loc_id, npc_id
                ));
            }
        }
    }

    // Warnings for potential issues
    if state.quests.is_empty() {
        warnings.push("Module has no quests defined.".into());
    }
    if state.npcs.is_empty() {
        warnings.push("Module has no NPCs defined.".into());
    }

    Ok(ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    })
}

#[tauri::command]
pub fn export_module(
    app: tauri::AppHandle,
    name: String,
    json: String,
) -> Result<ModuleExportResult, String> {
    // Validate first
    let validation = validate_module_json(json.clone())?;
    if !validation.valid {
        return Err(format!(
            "Module has errors: {}",
            validation.errors.join(", ")
        ));
    }

    let module_id = sanitize_module_name(&name)?;

    // Save to modules directory
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let modules_dir = app_data_dir.join("modules");
    std::fs::create_dir_all(&modules_dir).map_err(|e| e.to_string())?;

    let filepath = modules_dir.join(&module_id);

    // Pretty print JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let pretty = serde_json::to_string_pretty(&parsed).map_err(|e| e.to_string())?;

    std::fs::write(&filepath, pretty).map_err(|e| e.to_string())?;

    Ok(ModuleExportResult { module_id })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    fn valid_module_json() -> String {
        let state = world_builder::build_thornhold();
        serde_json::to_string(&state).unwrap()
    }

    #[test]
    fn validate_valid_module() {
        let json = valid_module_json();
        let result = validate_module_json(json).unwrap();
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn validate_invalid_json() {
        let result = validate_module_json("not json at all".into()).unwrap();
        assert!(!result.valid);
        assert!(result.errors[0].contains("Invalid JSON"));
    }

    #[test]
    fn validate_missing_start_location() {
        let mut state = world_builder::build_thornhold();
        state.player.location = "nonexistent_room".into();
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("does not exist")));
    }

    #[test]
    fn validate_dangling_exit_reference() {
        let mut state = world_builder::build_thornhold();
        if let Some(loc) = state.locations.get_mut("courtyard") {
            loc.exits
                .insert(crate::models::Direction::Up, "phantom_room".into());
        }
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("non-existent") && e.contains("phantom_room")));
    }

    #[test]
    fn validate_dangling_item_reference() {
        let mut state = world_builder::build_thornhold();
        if let Some(loc) = state.locations.get_mut("courtyard") {
            loc.items.push("ghost_item".into());
        }
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("non-existent item") && e.contains("ghost_item")));
    }

    #[test]
    fn validate_dangling_npc_reference() {
        let mut state = world_builder::build_thornhold();
        if let Some(loc) = state.locations.get_mut("courtyard") {
            loc.npcs.push("phantom_npc".into());
        }
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("non-existent NPC") && e.contains("phantom_npc")));
    }

    #[test]
    fn validate_empty_locations() {
        let state = WorldState::default();
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("at least one location")));
    }

    #[test]
    fn validate_warnings_no_quests_no_npcs() {
        let mut state = world_builder::build_thornhold();
        state.quests.clear();
        state.npcs.clear();
        // Also clear NPC refs from locations so it doesn't error
        for loc in state.locations.values_mut() {
            loc.npcs.clear();
        }
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(result.valid);
        assert!(result.warnings.iter().any(|w| w.contains("no quests")));
        assert!(result.warnings.iter().any(|w| w.contains("no NPCs")));
    }

    #[test]
    fn export_rejects_invalid_module() {
        // export_module requires an AppHandle which we can't easily construct in tests,
        // but we can verify the validation logic by testing validate_module_json
        let state = WorldState::default();
        let json = serde_json::to_string(&state).unwrap();
        let result = validate_module_json(json).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn sanitize_module_name_creates_safe_module_id() {
        assert_eq!(
            sanitize_module_name(" Thornhold Depths ").unwrap(),
            "thornhold_depths.json"
        );
        assert_eq!(
            sanitize_module_name("Temple@Night!").unwrap(),
            "temple_night.json"
        );
    }

    #[test]
    fn sanitize_module_name_rejects_path_like_input() {
        assert!(sanitize_module_name("../thornhold").is_err());
        assert!(sanitize_module_name("mods\\thornhold").is_err());
        assert!(sanitize_module_name("C:thornhold").is_err());
    }

    #[test]
    fn sanitize_module_name_rejects_empty_or_non_alnum_values() {
        assert!(sanitize_module_name("   ").is_err());
        assert!(sanitize_module_name("!!!").is_err());
    }
}
