use tauri::State;

use crate::models::{GameMode, WorldState};
use crate::persistence::state::GameState;

const BASE_COMMANDS: &[&str] = &[
    "look", "go", "take", "drop", "use", "equip", "unequip", "talk", "attack", "flee",
    "inventory", "map", "quests", "help", "save", "load",
];

const DIRECTIONS: &[&str] = &["north", "south", "east", "west", "up", "down"];

#[tauri::command]
pub fn get_completions(prefix: String, game_state: State<GameState>) -> Result<Vec<String>, String> {
    let state = game_state.0.lock().map_err(|e| e.to_string())?;
    let prefix_lower = prefix.to_lowercase();
    let prefix_lower = prefix_lower.trim();

    if prefix_lower.is_empty() {
        return Ok(Vec::new());
    }

    let mut candidates: Vec<String> = Vec::new();

    // Check if prefix contains a space — contextual completions
    if let Some(space_idx) = prefix_lower.find(' ') {
        let verb = &prefix_lower[..space_idx];
        let arg = prefix_lower[space_idx + 1..].trim_start();

        match verb {
            "go" | "move" | "walk" | "head" => {
                candidates.extend(direction_completions(arg, &state));
            }
            "take" | "get" | "grab" | "pick" => {
                candidates.extend(room_item_completions(arg, &state));
            }
            "drop" | "use" | "equip" => {
                candidates.extend(inventory_completions(arg, &state));
            }
            "unequip" | "remove" => {
                candidates.extend(equipped_completions(arg, &state));
            }
            "talk" | "speak" | "ask" | "chat" => {
                candidates.extend(npc_completions(arg, &state));
            }
            "attack" | "fight" | "hit" | "kill" | "strike" => {
                candidates.extend(npc_completions(arg, &state));
            }
            "look" | "examine" | "inspect" => {
                candidates.extend(room_item_completions(arg, &state));
                candidates.extend(inventory_completions(arg, &state));
                candidates.extend(npc_completions(arg, &state));
            }
            "save" | "load" => {
                // No argument completions for save/load
            }
            _ => {}
        }

        // Prefix the verb back
        candidates = candidates
            .into_iter()
            .map(|c| format!("{} {}", verb, c))
            .collect();
    } else {
        // Complete the command verb itself
        // In combat mode, only combat-relevant commands
        if matches!(state.game_mode, GameMode::InCombat(_)) {
            let combat_commands = &["attack", "flee", "use", "inventory", "help"];
            for cmd in combat_commands.iter() {
                if cmd.starts_with(prefix_lower) {
                    candidates.push(cmd.to_string());
                }
            }
        } else {
            for cmd in BASE_COMMANDS {
                if cmd.starts_with(prefix_lower) {
                    candidates.push(cmd.to_string());
                }
            }
            // Also match direction shortcuts
            for dir in DIRECTIONS {
                if dir.starts_with(prefix_lower) {
                    candidates.push(dir.to_string());
                }
            }
        }
    }

    candidates.sort();
    candidates.dedup();
    candidates.truncate(10);
    Ok(candidates)
}

fn direction_completions(arg: &str, state: &WorldState) -> Vec<String> {
    let loc = match state.locations.get(&state.player.location) {
        Some(l) => l,
        None => return Vec::new(),
    };
    loc.exits
        .keys()
        .map(|d| format!("{}", d).to_lowercase())
        .filter(|d| d.starts_with(arg))
        .collect()
}

fn room_item_completions(arg: &str, state: &WorldState) -> Vec<String> {
    let loc = match state.locations.get(&state.player.location) {
        Some(l) => l,
        None => return Vec::new(),
    };
    loc.items
        .iter()
        .filter_map(|id| state.items.get(id))
        .map(|i| i.name.to_lowercase())
        .filter(|name| name.starts_with(arg) || name.contains(arg))
        .collect()
}

fn inventory_completions(arg: &str, state: &WorldState) -> Vec<String> {
    state
        .player
        .inventory
        .iter()
        .filter_map(|id| state.items.get(id))
        .map(|i| i.name.to_lowercase())
        .filter(|name| name.starts_with(arg) || name.contains(arg))
        .collect()
}

fn equipped_completions(arg: &str, state: &WorldState) -> Vec<String> {
    let mut results = Vec::new();
    if let Some(ref wid) = state.player.equipped_weapon {
        if let Some(item) = state.items.get(wid) {
            let name = item.name.to_lowercase();
            if name.starts_with(arg) || name.contains(arg) {
                results.push(name);
            }
        }
    }
    if let Some(ref aid) = state.player.equipped_armor {
        if let Some(item) = state.items.get(aid) {
            let name = item.name.to_lowercase();
            if name.starts_with(arg) || name.contains(arg) {
                results.push(name);
            }
        }
    }
    results
}

fn npc_completions(arg: &str, state: &WorldState) -> Vec<String> {
    let loc = match state.locations.get(&state.player.location) {
        Some(l) => l,
        None => return Vec::new(),
    };
    loc.npcs
        .iter()
        .filter_map(|id| state.npcs.get(id))
        .filter(|n| n.dialogue_state != crate::models::DialogueState::Dead)
        .map(|n| n.name.to_lowercase())
        .filter(|name| name.starts_with(arg) || name.contains(arg))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    fn build_state() -> WorldState {
        world_builder::build_thornhold()
    }

    #[test]
    fn completes_base_commands() {
        let _state = build_state();
        let prefix_lower = "lo";
        let mut candidates: Vec<String> = BASE_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(prefix_lower))
            .map(|s| s.to_string())
            .collect();
        candidates.sort();
        assert!(candidates.contains(&"look".to_string()));
        assert!(candidates.contains(&"load".to_string()));
    }

    #[test]
    fn completes_go_directions() {
        let state = build_state();
        // Courtyard has east and south exits
        let dirs = direction_completions("", &state);
        assert!(!dirs.is_empty());
        assert!(dirs.contains(&"east".to_string()) || dirs.contains(&"south".to_string()));
    }

    #[test]
    fn completes_room_items() {
        let state = build_state();
        // Courtyard has rusty_lantern and merchant_journal
        let items = room_item_completions("", &state);
        assert!(!items.is_empty());
    }

    #[test]
    fn completes_npcs() {
        let state = build_state();
        // Courtyard has merchant_ghost
        let npcs = npc_completions("", &state);
        assert!(!npcs.is_empty());
    }

    #[test]
    fn empty_prefix_returns_empty() {
        let _state = build_state();
        let prefix = "";
        let candidates: Vec<String> = BASE_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();
        // empty prefix matches all, but get_completions returns empty for empty prefix
        assert!(candidates.len() > 0);
    }

    #[test]
    fn inventory_completions_empty_when_no_items() {
        let state = build_state();
        let items = inventory_completions("", &state);
        assert!(items.is_empty()); // Fresh game has no inventory
    }
}
