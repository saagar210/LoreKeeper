use crate::models::{ActionType, WorldState};

/// Check which achievements should be unlocked based on current state and last action.
/// Returns IDs of newly-eligible achievements (caller must check DB for already-unlocked).
pub fn check_achievements(state: &WorldState, action_type: &ActionType) -> Vec<String> {
    let mut earned = Vec::new();

    // First Blood: defeat first enemy
    if matches!(action_type, ActionType::CombatVictory { .. }) {
        earned.push("first_blood".to_string());
    }

    // Explorer: visit 8+ locations
    if state.player.visited_locations.len() >= 8 {
        earned.push("explorer".to_string());
    }

    // Completionist: all quests completed
    if !state.quests.is_empty() && state.quests.values().all(|q| q.completed) {
        earned.push("completionist".to_string());
    }

    // Speedrunner: reach final_sanctum in under 30 turns
    if state.player.location == "final_sanctum" && state.player.turns_elapsed < 30 {
        earned.push("speedrunner".to_string());
    }

    // Hoarder: 8+ items in inventory
    if state.player.inventory.len() >= 8 {
        earned.push("hoarder".to_string());
    }

    // Secret Keeper: discovered 3+ secret commands
    if state.player.discovered_secrets.len() >= 3 {
        earned.push("bookworm".to_string());
    }

    // Survivor: survive with 5 or fewer HP (must be alive)
    if state.player.health > 0 && state.player.health <= 5 {
        earned.push("survivor".to_string());
    }

    // Diplomat: complete game through negotiation (peace ending)
    if matches!(
        &state.game_mode,
        crate::models::GameMode::GameOver(crate::models::EndingType::VictoryPeace)
    ) {
        earned.push("diplomat".to_string());
    }

    earned
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    #[test]
    fn first_blood_on_combat_victory() {
        let state = world_builder::build_thornhold();
        let earned = check_achievements(
            &state,
            &ActionType::CombatVictory {
                enemy_name: "Goblin".into(),
            },
        );
        assert!(earned.contains(&"first_blood".to_string()));
    }

    #[test]
    fn explorer_needs_8_locations() {
        let mut state = world_builder::build_thornhold();
        for i in 0..8 {
            state
                .player
                .visited_locations
                .insert(format!("loc_{}", i));
        }
        let earned = check_achievements(&state, &ActionType::DisplayOnly);
        assert!(earned.contains(&"explorer".to_string()));
    }

    #[test]
    fn hoarder_needs_8_items() {
        let mut state = world_builder::build_thornhold();
        for i in 0..8 {
            state.player.inventory.push(format!("item_{}", i));
        }
        let earned = check_achievements(&state, &ActionType::DisplayOnly);
        assert!(earned.contains(&"hoarder".to_string()));
    }

    #[test]
    fn survivor_at_low_hp() {
        let mut state = world_builder::build_thornhold();
        state.player.health = 3;
        let earned = check_achievements(&state, &ActionType::DisplayOnly);
        assert!(earned.contains(&"survivor".to_string()));
    }

    #[test]
    fn diplomat_on_peace_ending() {
        let mut state = world_builder::build_thornhold();
        state.game_mode =
            crate::models::GameMode::GameOver(crate::models::EndingType::VictoryPeace);
        let earned = check_achievements(&state, &ActionType::DisplayOnly);
        assert!(earned.contains(&"diplomat".to_string()));
    }
}
