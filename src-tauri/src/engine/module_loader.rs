use std::path::Path;

use crate::models::WorldState;

pub const MAX_MODULE_FILE_BYTES: usize = 512 * 1024;

const MAX_LOCATIONS: usize = 128;
const MAX_ITEMS: usize = 512;
const MAX_NPCS: usize = 256;
const MAX_QUESTS: usize = 128;
const MAX_EVENTS: usize = 256;
const MAX_RECIPES: usize = 128;
const MAX_JOURNAL_ENTRIES: usize = 256;
const MAX_DIALOGUE_HISTORY_ENTRIES: usize = 256;
const MAX_COMMAND_LOG_ENTRIES: usize = 512;
const MAX_COMBAT_LOG_ENTRIES: usize = 256;
const MAX_MESSAGE_LOG_ENTRIES: usize = 256;
const MAX_VISITED_LOCATIONS: usize = 256;
const MAX_PLAYER_INVENTORY: usize = 64;
const MAX_PLAYER_MAX_INVENTORY: usize = 128;
const MAX_LOCATION_ITEMS: usize = 64;
const MAX_LOCATION_NPCS: usize = 32;
const MAX_NPC_ITEMS: usize = 32;
const MAX_NPC_MEMORY_ENTRIES: usize = 64;
const MAX_QUEST_REWARDS: usize = 16;
const MAX_RECIPE_INPUTS: usize = 8;
const MAX_ID_LEN: usize = 64;
const MAX_SHORT_TEXT_LEN: usize = 120;
const MAX_LONG_TEXT_LEN: usize = 4_000;
const MAX_HINT_LEN: usize = 300;
const MAX_MEMORY_EVENT_LEN: usize = 240;
const MAX_COMMAND_INPUT_LEN: usize = 240;
const MAX_PERSONALITY_SEED_LEN: usize = 240;

pub fn ensure_module_json_size(json: &str) -> Result<(), String> {
    let bytes = json.len();
    if bytes == 0 {
        return Err("Module file cannot be empty.".into());
    }
    if bytes > MAX_MODULE_FILE_BYTES {
        return Err(format!(
            "Module exceeds the size limit of {} KB.",
            MAX_MODULE_FILE_BYTES / 1024
        ));
    }
    Ok(())
}

pub fn parse_module_json(json: &str) -> Result<WorldState, String> {
    ensure_module_json_size(json)?;
    serde_json::from_str(json).map_err(|e| format!("Invalid module JSON: {}", e))
}

pub fn inspect_module(path: &Path) -> Result<WorldState, String> {
    let metadata =
        std::fs::metadata(path).map_err(|e| format!("Failed to inspect module file: {}", e))?;
    if !metadata.is_file() {
        return Err("Module path must point to a file.".into());
    }
    let file_len = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    if file_len == 0 {
        return Err("Module file cannot be empty.".into());
    }
    if file_len > MAX_MODULE_FILE_BYTES {
        return Err(format!(
            "Module exceeds the size limit of {} KB.",
            MAX_MODULE_FILE_BYTES / 1024
        ));
    }

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read module: {}", e))?;
    let state = parse_module_json(&content)?;
    validate_module_state(&state)?;
    Ok(state)
}

pub fn load_module(path: &Path) -> Result<WorldState, String> {
    let mut state = inspect_module(path)?;
    state.initialized = true;

    Ok(state)
}

pub fn validate_module_state(state: &WorldState) -> Result<(), String> {
    if state.locations.is_empty() {
        return Err("Module must have at least one location.".into());
    }

    check_count("locations", state.locations.len(), MAX_LOCATIONS)?;
    check_count("items", state.items.len(), MAX_ITEMS)?;
    check_count("npcs", state.npcs.len(), MAX_NPCS)?;
    check_count("quests", state.quests.len(), MAX_QUESTS)?;
    check_count("events", state.events.len(), MAX_EVENTS)?;
    check_count("recipes", state.recipes.len(), MAX_RECIPES)?;
    check_count("journal entries", state.journal.len(), MAX_JOURNAL_ENTRIES)?;
    check_count(
        "dialogue history entries",
        state.dialogue_history.len(),
        MAX_DIALOGUE_HISTORY_ENTRIES,
    )?;
    check_count(
        "command log entries",
        state.command_log.len(),
        MAX_COMMAND_LOG_ENTRIES,
    )?;
    check_count(
        "combat log entries",
        state.combat_log.len(),
        MAX_COMBAT_LOG_ENTRIES,
    )?;
    check_count(
        "message log entries",
        state.message_log.len(),
        MAX_MESSAGE_LOG_ENTRIES,
    )?;

    check_string_len("player.location", &state.player.location, MAX_ID_LEN)?;
    check_count(
        "player.inventory entries",
        state.player.inventory.len(),
        MAX_PLAYER_INVENTORY,
    )?;
    check_count(
        "player.visitedLocations",
        state.player.visited_locations.len(),
        MAX_VISITED_LOCATIONS,
    )?;
    check_count(
        "player.statusEffects",
        state.player.status_effects.len(),
        MAX_PLAYER_INVENTORY,
    )?;
    check_count(
        "player.discoveredSecrets",
        state.player.discovered_secrets.len(),
        MAX_VISITED_LOCATIONS,
    )?;
    if state.player.max_inventory > MAX_PLAYER_MAX_INVENTORY {
        return Err(format!(
            "Player max inventory exceeds limit of {}.",
            MAX_PLAYER_MAX_INVENTORY
        ));
    }
    if state.player.inventory.len() > state.player.max_inventory {
        return Err("Player inventory exceeds max inventory.".into());
    }

    // Player start location must exist
    if !state.locations.contains_key(&state.player.location) {
        return Err(format!(
            "Player start location '{}' not found in locations.",
            state.player.location
        ));
    }

    for (location_key, location) in &state.locations {
        check_string_len(
            format!("location key '{}'", location_key),
            location_key,
            MAX_ID_LEN,
        )?;
        check_string_len(
            format!("location '{}'.id", location_key),
            &location.id,
            MAX_ID_LEN,
        )?;
        if location.id != *location_key {
            return Err(format!(
                "Location map key '{}' must match location id '{}'.",
                location_key, location.id
            ));
        }
        check_string_len(
            format!("location '{}'.name", location_key),
            &location.name,
            MAX_SHORT_TEXT_LEN,
        )?;
        check_string_len(
            format!("location '{}'.description", location_key),
            &location.description,
            MAX_LONG_TEXT_LEN,
        )?;
        check_optional_string_len(
            format!("location '{}'.examineDetails", location_key),
            location.examine_details.as_deref(),
            MAX_LONG_TEXT_LEN,
        )?;
        check_optional_string_len(
            format!("location '{}'.revisitDescription", location_key),
            location.revisit_description.as_deref(),
            MAX_LONG_TEXT_LEN,
        )?;
        check_count(
            format!("location '{}'.items", location_key),
            location.items.len(),
            MAX_LOCATION_ITEMS,
        )?;
        check_count(
            format!("location '{}'.npcs", location_key),
            location.npcs.len(),
            MAX_LOCATION_NPCS,
        )?;
        if location.exits.len() > 6 {
            return Err(format!(
                "Location '{}' has too many exits; maximum is 6.",
                location_key
            ));
        }
        if location.locked_exits.len() > 6 {
            return Err(format!(
                "Location '{}' has too many locked exits; maximum is 6.",
                location_key
            ));
        }
        for secret in &location.discovered_secrets {
            check_string_len(
                format!("location '{}'.discoveredSecrets entry", location_key),
                secret,
                MAX_ID_LEN,
            )?;
        }
    }

    // All exits must point to valid locations
    for (loc_id, loc) in &state.locations {
        for (dir, target) in &loc.exits {
            check_string_len(
                format!("location '{}'.exit {:?}", loc_id, dir),
                target,
                MAX_ID_LEN,
            )?;
            if !state.locations.contains_key(target) {
                return Err(format!(
                    "Location '{}' has exit {:?} to '{}' which doesn't exist.",
                    loc_id, dir, target
                ));
            }
        }
        // locked_exits maps Direction -> key_id (item needed to unlock)
        // Keys may be quest rewards or event-granted, so we don't validate them
        for key_id in loc.locked_exits.values() {
            check_string_len(
                format!("location '{}'.lockedExits key id", loc_id),
                key_id,
                MAX_ID_LEN,
            )?;
        }
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

    for item_id in &state.player.inventory {
        check_string_len("player.inventory item id", item_id, MAX_ID_LEN)?;
        if !state.items.contains_key(item_id) {
            return Err(format!(
                "Player inventory references item '{}' which doesn't exist.",
                item_id
            ));
        }
    }
    if let Some(weapon_id) = state.player.equipped_weapon.as_deref() {
        check_string_len("player.equippedWeapon", weapon_id, MAX_ID_LEN)?;
        if !state.items.contains_key(weapon_id) {
            return Err(format!(
                "Player equipped weapon '{}' which doesn't exist.",
                weapon_id
            ));
        }
    }
    if let Some(armor_id) = state.player.equipped_armor.as_deref() {
        check_string_len("player.equippedArmor", armor_id, MAX_ID_LEN)?;
        if !state.items.contains_key(armor_id) {
            return Err(format!(
                "Player equipped armor '{}' which doesn't exist.",
                armor_id
            ));
        }
    }

    for (item_key, item) in &state.items {
        check_string_len(format!("item key '{}'", item_key), item_key, MAX_ID_LEN)?;
        check_string_len(format!("item '{}'.id", item_key), &item.id, MAX_ID_LEN)?;
        if item.id != *item_key {
            return Err(format!(
                "Item map key '{}' must match item id '{}'.",
                item_key, item.id
            ));
        }
        check_string_len(
            format!("item '{}'.name", item_key),
            &item.name,
            MAX_SHORT_TEXT_LEN,
        )?;
        check_string_len(
            format!("item '{}'.description", item_key),
            &item.description,
            MAX_LONG_TEXT_LEN,
        )?;
        check_optional_string_len(
            format!("item '{}'.keyId", item_key),
            item.key_id.as_deref(),
            MAX_ID_LEN,
        )?;
        check_optional_string_len(
            format!("item '{}'.lore", item_key),
            item.lore.as_deref(),
            MAX_LONG_TEXT_LEN,
        )?;
    }

    for (npc_key, npc) in &state.npcs {
        check_string_len(format!("npc key '{}'", npc_key), npc_key, MAX_ID_LEN)?;
        check_string_len(format!("npc '{}'.id", npc_key), &npc.id, MAX_ID_LEN)?;
        if npc.id != *npc_key {
            return Err(format!(
                "NPC map key '{}' must match npc id '{}'.",
                npc_key, npc.id
            ));
        }
        check_string_len(
            format!("npc '{}'.name", npc_key),
            &npc.name,
            MAX_SHORT_TEXT_LEN,
        )?;
        check_string_len(
            format!("npc '{}'.description", npc_key),
            &npc.description,
            MAX_LONG_TEXT_LEN,
        )?;
        check_string_len(
            format!("npc '{}'.personalitySeed", npc_key),
            &npc.personality_seed,
            MAX_PERSONALITY_SEED_LEN,
        )?;
        check_optional_string_len(
            format!("npc '{}'.questGiver", npc_key),
            npc.quest_giver.as_deref(),
            MAX_ID_LEN,
        )?;
        check_optional_string_len(
            format!("npc '{}'.examineText", npc_key),
            npc.examine_text.as_deref(),
            MAX_LONG_TEXT_LEN,
        )?;
        check_count(
            format!("npc '{}'.items", npc_key),
            npc.items.len(),
            MAX_NPC_ITEMS,
        )?;
        check_count(
            format!("npc '{}'.memory", npc_key),
            npc.memory.len(),
            MAX_NPC_MEMORY_ENTRIES,
        )?;
        for item_id in &npc.items {
            check_string_len(format!("npc '{}'.item id", npc_key), item_id, MAX_ID_LEN)?;
            if !state.items.contains_key(item_id) {
                return Err(format!(
                    "NPC '{}' references item '{}' which doesn't exist.",
                    npc_key, item_id
                ));
            }
        }
        for memory in &npc.memory {
            check_string_len(
                format!("npc '{}'.memory event", npc_key),
                &memory.event,
                MAX_MEMORY_EVENT_LEN,
            )?;
        }
    }

    for (quest_key, quest) in &state.quests {
        check_string_len(format!("quest key '{}'", quest_key), quest_key, MAX_ID_LEN)?;
        check_string_len(format!("quest '{}'.id", quest_key), &quest.id, MAX_ID_LEN)?;
        if quest.id != *quest_key {
            return Err(format!(
                "Quest map key '{}' must match quest id '{}'.",
                quest_key, quest.id
            ));
        }
        check_string_len(
            format!("quest '{}'.name", quest_key),
            &quest.name,
            MAX_SHORT_TEXT_LEN,
        )?;
        check_string_len(
            format!("quest '{}'.description", quest_key),
            &quest.description,
            MAX_LONG_TEXT_LEN,
        )?;
        check_string_len(
            format!("quest '{}'.giver", quest_key),
            &quest.giver,
            MAX_ID_LEN,
        )?;
        if !state.npcs.contains_key(&quest.giver) {
            return Err(format!(
                "Quest '{}' references giver '{}' which doesn't exist.",
                quest_key, quest.giver
            ));
        }
        check_count(
            format!("quest '{}'.reward", quest_key),
            quest.reward.len(),
            MAX_QUEST_REWARDS,
        )?;
        for reward_id in &quest.reward {
            check_string_len(
                format!("quest '{}'.reward item", quest_key),
                reward_id,
                MAX_ID_LEN,
            )?;
            if !state.items.contains_key(reward_id) {
                return Err(format!(
                    "Quest '{}' reward item '{}' doesn't exist.",
                    quest_key, reward_id
                ));
            }
        }
        match &quest.objective {
            crate::models::QuestObjective::FetchItem(item_id) => {
                check_string_len(
                    format!("quest '{}'.objective.fetchItem", quest_key),
                    item_id,
                    MAX_ID_LEN,
                )?;
                if !state.items.contains_key(item_id) {
                    return Err(format!(
                        "Quest '{}' objective item '{}' doesn't exist.",
                        quest_key, item_id
                    ));
                }
            }
            crate::models::QuestObjective::KillNpc(npc_id) => {
                check_string_len(
                    format!("quest '{}'.objective.killNpc", quest_key),
                    npc_id,
                    MAX_ID_LEN,
                )?;
                if !state.npcs.contains_key(npc_id) {
                    return Err(format!(
                        "Quest '{}' objective NPC '{}' doesn't exist.",
                        quest_key, npc_id
                    ));
                }
            }
            crate::models::QuestObjective::ReachLocation(location_id) => {
                check_string_len(
                    format!("quest '{}'.objective.reachLocation", quest_key),
                    location_id,
                    MAX_ID_LEN,
                )?;
                if !state.locations.contains_key(location_id) {
                    return Err(format!(
                        "Quest '{}' objective location '{}' doesn't exist.",
                        quest_key, location_id
                    ));
                }
            }
        }
    }

    for recipe in &state.recipes {
        check_string_len("recipe.id", &recipe.id, MAX_ID_LEN)?;
        check_count("recipe.inputs", recipe.inputs.len(), MAX_RECIPE_INPUTS)?;
        check_string_len("recipe.output", &recipe.output, MAX_ID_LEN)?;
        if !state.items.contains_key(&recipe.output) {
            return Err(format!(
                "Recipe '{}' output '{}' doesn't exist.",
                recipe.id, recipe.output
            ));
        }
        for input_id in &recipe.inputs {
            check_string_len("recipe.input", input_id, MAX_ID_LEN)?;
            if !state.items.contains_key(input_id) {
                return Err(format!(
                    "Recipe '{}' input '{}' doesn't exist.",
                    recipe.id, input_id
                ));
            }
        }
        check_string_len("recipe.hint", &recipe.hint, MAX_HINT_LEN)?;
    }

    for entry in &state.journal {
        check_string_len("journal entry.id", &entry.id, MAX_ID_LEN)?;
        check_string_len("journal entry.title", &entry.title, MAX_SHORT_TEXT_LEN)?;
        check_string_len("journal entry.content", &entry.content, MAX_LONG_TEXT_LEN)?;
    }

    for entry in &state.dialogue_history {
        check_string_len("dialogue history role", &entry.role, MAX_SHORT_TEXT_LEN)?;
        check_string_len("dialogue history text", &entry.text, MAX_LONG_TEXT_LEN)?;
    }

    for message in &state.message_log {
        check_string_len("message log entry", message, MAX_LONG_TEXT_LEN)?;
    }

    for entry in &state.command_log {
        check_string_len("command log input", &entry.input, MAX_COMMAND_INPUT_LEN)?;
        check_string_len("command log location", &entry.location, MAX_ID_LEN)?;
    }

    for entry in &state.combat_log {
        check_string_len("combat log attacker", &entry.attacker, MAX_SHORT_TEXT_LEN)?;
        check_string_len("combat log defender", &entry.defender, MAX_SHORT_TEXT_LEN)?;
    }

    if let Some(context) = &state.last_narrative_context {
        check_string_len(
            "narrative context locationName",
            &context.location_name,
            MAX_SHORT_TEXT_LEN,
        )?;
        check_string_len(
            "narrative context locationDescription",
            &context.location_description,
            MAX_LONG_TEXT_LEN,
        )?;
        check_string_len("narrative context mood", &context.mood, MAX_SHORT_TEXT_LEN)?;
        check_count(
            "narrative context inventoryNames",
            context.inventory_names.len(),
            MAX_PLAYER_INVENTORY,
        )?;
        check_count(
            "narrative context roomItemNames",
            context.room_item_names.len(),
            MAX_LOCATION_ITEMS,
        )?;
        check_count(
            "narrative context roomNpcNames",
            context.room_npc_names.len(),
            MAX_LOCATION_NPCS,
        )?;
    }

    Ok(())
}

fn check_count(label: impl AsRef<str>, count: usize, max: usize) -> Result<(), String> {
    if count > max {
        return Err(format!("{} exceeds limit of {}.", label.as_ref(), max));
    }
    Ok(())
}

fn check_string_len(label: impl AsRef<str>, value: &str, max: usize) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} cannot be empty.", label.as_ref()));
    }
    if value.len() > max {
        return Err(format!("{} exceeds {} characters.", label.as_ref(), max));
    }
    Ok(())
}

fn check_optional_string_len(
    label: impl AsRef<str>,
    value: Option<&str>,
    max: usize,
) -> Result<(), String> {
    if let Some(value) = value {
        check_string_len(label, value, max)?;
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
        assert!(validate_module_state(&state).is_ok());
    }

    #[test]
    fn invalid_start_location_rejected() {
        let mut state = world_builder::build_thornhold();
        state.player.location = "nonexistent".into();
        let result = validate_module_state(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn invalid_exit_rejected() {
        let mut state = world_builder::build_thornhold();
        if let Some(loc) = state.locations.get_mut("courtyard") {
            loc.exits
                .insert(crate::models::Direction::Up, "does_not_exist".into());
        }
        let result = validate_module_state(&state);
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

    #[test]
    fn rejects_oversized_module_json() {
        let oversized = "x".repeat(MAX_MODULE_FILE_BYTES + 1);
        let result = parse_module_json(&oversized);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("size limit"));
    }

    #[test]
    fn rejects_module_with_too_many_locations() {
        let mut state = world_builder::build_thornhold();
        for index in 0..(MAX_LOCATIONS + 1) {
            let id = format!("extra_room_{index}");
            state.locations.insert(
                id.clone(),
                crate::models::Location {
                    id: id.clone(),
                    name: format!("Room {index}"),
                    description: "A safe test room.".into(),
                    items: vec![],
                    npcs: vec![],
                    exits: std::collections::HashMap::new(),
                    locked_exits: std::collections::HashMap::new(),
                    visited: false,
                    discovered_secrets: vec![],
                    ambient_mood: crate::models::Mood::Peaceful,
                    examine_details: None,
                    revisit_description: None,
                },
            );
        }

        let result = validate_module_state(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("locations exceeds limit"));
    }

    #[test]
    fn rejects_module_with_overlong_description() {
        let mut state = world_builder::build_thornhold();
        state.locations.get_mut("courtyard").unwrap().description =
            "x".repeat(MAX_LONG_TEXT_LEN + 1);

        let result = validate_module_state(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("description exceeds"));
    }
}
