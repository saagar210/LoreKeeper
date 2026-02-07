use crate::models::*;

pub fn check_quest_progress(state: &mut WorldState) -> Vec<OutputLine> {
    let mut messages = Vec::new();

    let quests: Vec<(String, Quest)> = state
        .quests
        .iter()
        .filter(|(_, q)| q.active && !q.completed)
        .map(|(id, q)| (id.clone(), q.clone()))
        .collect();

    for (quest_id, quest) in quests {
        let completed = match &quest.objective {
            QuestObjective::FetchItem(item_id) => {
                state.player.inventory.contains(item_id)
            }
            QuestObjective::KillNpc(npc_id) => {
                state
                    .npcs
                    .get(npc_id)
                    .map(|n| n.dialogue_state == DialogueState::Dead)
                    .unwrap_or(false)
            }
            QuestObjective::ReachLocation(loc_id) => {
                state.player.location == *loc_id
            }
        };

        if completed {
            if let Some(q) = state.quests.get_mut(&quest_id) {
                q.completed = true;
            }

            // Check if quest giver is nearby for auto-complete
            let giver_nearby = state
                .locations
                .get(&state.player.location)
                .map(|loc| loc.npcs.contains(&quest.giver))
                .unwrap_or(false);

            if giver_nearby {
                // Auto-transition NPC to QuestComplete state
                if let Some(npc) = state.npcs.get_mut(&quest.giver) {
                    npc.dialogue_state = DialogueState::QuestComplete;
                }
                let giver_name = state
                    .npcs
                    .get(&quest.giver)
                    .map(|n| n.name.as_str())
                    .unwrap_or(&quest.giver);
                messages.push(OutputLine {
                    text: format!(
                        "Quest objective complete! {} wants to speak with you.",
                        giver_name
                    ),
                    line_type: LineType::System,
                });
            } else {
                messages.push(OutputLine {
                    text: format!(
                        "Quest objective complete! Return to {} to claim your reward.",
                        state
                            .npcs
                            .get(&quest.giver)
                            .map(|n| n.name.as_str())
                            .unwrap_or(&quest.giver)
                    ),
                    line_type: LineType::System,
                });
            }
        }
    }

    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn fetch_item_quest_completes() {
        let mut state = WorldState::default();
        state.quests.insert(
            "fetch_quest".into(),
            Quest {
                id: "fetch_quest".into(),
                name: "Fetch Quest".into(),
                description: "Get the item.".into(),
                giver: "npc1".into(),
                objective: QuestObjective::FetchItem("magic_item".into()),
                reward: vec![],
                completed: false,
                active: true,
            },
        );
        state.npcs.insert(
            "npc1".into(),
            Npc {
                id: "npc1".into(),
                name: "Quest Giver".into(),
                description: "An NPC.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::QuestActive,
                hostile: false,
                health: 1,
                max_health: 1,
                attack: 0,
                defense: 0,
                items: vec![],
                quest_giver: Some("fetch_quest".into()),
            },
        );
        state.locations.insert(
            "courtyard".into(),
            Location {
                id: "courtyard".into(),
                name: "Courtyard".into(),
                description: "Start.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::new(),
                locked_exits: HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );

        // Quest not complete yet
        let msgs = check_quest_progress(&mut state);
        assert!(msgs.is_empty());
        assert!(!state.quests.get("fetch_quest").unwrap().completed);

        // Player picks up the item
        state.player.inventory.push("magic_item".into());
        let msgs = check_quest_progress(&mut state);
        assert!(!msgs.is_empty());
        assert!(state.quests.get("fetch_quest").unwrap().completed);
    }

    #[test]
    fn kill_npc_quest_completes() {
        let mut state = WorldState::default();
        state.quests.insert(
            "kill_quest".into(),
            Quest {
                id: "kill_quest".into(),
                name: "Kill Quest".into(),
                description: "Kill the enemy.".into(),
                giver: "npc1".into(),
                objective: QuestObjective::KillNpc("enemy".into()),
                reward: vec![],
                completed: false,
                active: true,
            },
        );
        state.npcs.insert(
            "enemy".into(),
            Npc {
                id: "enemy".into(),
                name: "Enemy".into(),
                description: "Bad.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::Hostile,
                hostile: true,
                health: 10,
                max_health: 10,
                attack: 5,
                defense: 2,
                items: vec![],
                quest_giver: None,
            },
        );
        state.npcs.insert(
            "npc1".into(),
            Npc {
                id: "npc1".into(),
                name: "Quest Giver".into(),
                description: "NPC.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::QuestActive,
                hostile: false,
                health: 1,
                max_health: 1,
                attack: 0,
                defense: 0,
                items: vec![],
                quest_giver: Some("kill_quest".into()),
            },
        );
        state.locations.insert(
            "courtyard".into(),
            Location {
                id: "courtyard".into(),
                name: "Courtyard".into(),
                description: "Start.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::new(),
                locked_exits: HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );

        // Not complete yet
        let msgs = check_quest_progress(&mut state);
        assert!(msgs.is_empty());

        // Kill the enemy
        state.npcs.get_mut("enemy").unwrap().dialogue_state = DialogueState::Dead;
        let msgs = check_quest_progress(&mut state);
        assert!(!msgs.is_empty());
        assert!(state.quests.get("kill_quest").unwrap().completed);
    }
}
