use std::path::Path;

use crate::models::WorldState;

pub fn load_module(path: &Path) -> Result<WorldState, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read module: {}", e))?;

    let mut state: WorldState =
        serde_json::from_str(&content).map_err(|e| format!("Invalid module JSON: {}", e))?;

    validate_module(&state)?;
    state.initialized = true;

    Ok(state)
}

fn validate_module(state: &WorldState) -> Result<(), String> {
    // Player start location must exist
    if !state.locations.contains_key(&state.player.location) {
        return Err(format!(
            "Player start location '{}' not found in locations.",
            state.player.location
        ));
    }

    // All exits must point to valid locations
    for (loc_id, loc) in &state.locations {
        for (dir, target) in &loc.exits {
            if !state.locations.contains_key(target) {
                return Err(format!(
                    "Location '{}' has exit {:?} to '{}' which doesn't exist.",
                    loc_id, dir, target
                ));
            }
        }
        // locked_exits maps Direction -> key_id (item needed to unlock)
        // Keys may be quest rewards or event-granted, so we don't validate them
    }

    // NPC references in locations must exist
    for (loc_id, loc) in &state.locations {
        for npc_id in &loc.npcs {
            if !state.npcs.contains_key(npc_id) {
                return Err(format!(
                    "Location '{}' references NPC '{}' which doesn't exist.",
                    loc_id, npc_id
                ));
            }
        }
    }

    // Item references in locations must exist
    for (loc_id, loc) in &state.locations {
        for item_id in &loc.items {
            if !state.items.contains_key(item_id) {
                return Err(format!(
                    "Location '{}' references item '{}' which doesn't exist.",
                    loc_id, item_id
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    #[test]
    fn thornhold_validates() {
        let state = world_builder::build_thornhold();
        assert!(validate_module(&state).is_ok());
    }

    #[test]
    fn invalid_start_location_rejected() {
        let mut state = WorldState::default();
        state.player.location = "nonexistent".into();
        let result = validate_module(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn invalid_exit_rejected() {
        let mut state = world_builder::build_thornhold();
        if let Some(loc) = state.locations.get_mut("courtyard") {
            loc.exits.insert(
                crate::models::Direction::Up,
                "does_not_exist".into(),
            );
        }
        let result = validate_module(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("doesn't exist"));
    }

    #[test]
    fn load_module_from_file() {
        let state = world_builder::build_thornhold();
        let json = serde_json::to_string_pretty(&state).unwrap();
        let tmp = std::env::temp_dir().join("test_module.json");
        std::fs::write(&tmp, &json).unwrap();

        let loaded = load_module(&tmp).unwrap();
        assert_eq!(loaded.locations.len(), state.locations.len());
        assert!(loaded.initialized);

        std::fs::remove_file(&tmp).ok();
    }
}
