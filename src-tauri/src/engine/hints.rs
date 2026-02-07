use crate::models::WorldState;

/// Returns a contextual hint for new players. Returns None once the player
/// has visited 4+ locations (experienced enough).
pub fn get_contextual_hint(state: &WorldState) -> Option<String> {
    // Stop hinting once player is experienced
    if state.player.visited_locations.len() >= 4 {
        return None;
    }

    // In combat: provide combat hints (highest priority)
    if matches!(state.game_mode, crate::models::GameMode::InCombat(_)) {
        return Some("Type 'attack' to fight or 'flee' to run away.".to_string());
    }

    let turns = state.player.turns_elapsed;

    // Turn 0: basic hint
    if turns == 0 {
        return Some("Type 'look' to examine your surroundings.".to_string());
    }

    // Early game: haven't picked up any items
    if turns <= 2 && state.player.inventory.is_empty() {
        return Some("Try 'take <item>' to pick up items you see.".to_string());
    }

    // First NPC encountered and never talked to anyone
    let loc = state.locations.get(&state.player.location)?;
    if !loc.npcs.is_empty() && turns <= 5 {
        let has_talked = state.npcs.values().any(|n| {
            matches!(
                n.dialogue_state,
                crate::models::DialogueState::Familiar
                    | crate::models::DialogueState::QuestActive
                    | crate::models::DialogueState::QuestComplete
            )
        });
        if !has_talked {
            return Some("Type 'talk to <name>' to speak with NPCs.".to_string());
        }
    }

    // First locked exit encountered
    if !loc.locked_exits.is_empty() {
        return Some("This exit is locked. Find the right key.".to_string());
    }

    // No active quests after a few turns
    if turns >= 5 {
        let has_active_quest = state.quests.values().any(|q| q.active && !q.completed);
        if !has_active_quest {
            return Some("Talk to NPCs — they may have quests for you.".to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    #[test]
    fn hint_on_turn_zero() {
        let state = world_builder::build_thornhold();
        let hint = get_contextual_hint(&state);
        assert_eq!(hint, Some("Type 'look' to examine your surroundings.".to_string()));
    }

    #[test]
    fn hint_take_items_early() {
        let mut state = world_builder::build_thornhold();
        state.player.turns_elapsed = 1;
        let hint = get_contextual_hint(&state);
        assert_eq!(hint, Some("Try 'take <item>' to pick up items you see.".to_string()));
    }

    #[test]
    fn hint_talk_to_npc() {
        let mut state = world_builder::build_thornhold();
        state.player.turns_elapsed = 3;
        state.player.inventory.push("rusty_lantern".into());
        let hint = get_contextual_hint(&state);
        assert_eq!(hint, Some("Type 'talk to <name>' to speak with NPCs.".to_string()));
    }

    #[test]
    fn no_hint_after_many_locations() {
        let mut state = world_builder::build_thornhold();
        state.player.visited_locations.insert("a".into());
        state.player.visited_locations.insert("b".into());
        state.player.visited_locations.insert("c".into());
        state.player.visited_locations.insert("d".into());
        let hint = get_contextual_hint(&state);
        assert!(hint.is_none());
    }

    #[test]
    fn hint_combat() {
        let mut state = world_builder::build_thornhold();
        state.game_mode = crate::models::GameMode::InCombat("goblin".into());
        let hint = get_contextual_hint(&state);
        assert_eq!(hint, Some("Type 'attack' to fight or 'flee' to run away.".to_string()));
    }

    #[test]
    fn hint_no_quests_after_turn_5() {
        let mut state = world_builder::build_thornhold();
        state.player.turns_elapsed = 6;
        state.player.inventory.push("something".into());
        // Move to a location with no NPCs and no locked exits
        state.player.location = "tower_apex".into();
        // Deactivate all quests
        for quest in state.quests.values_mut() {
            quest.active = false;
        }
        let hint = get_contextual_hint(&state);
        assert_eq!(hint, Some("Talk to NPCs — they may have quests for you.".to_string()));
    }
}
