use crate::models::*;

pub struct DialogueResult {
    pub messages: Vec<OutputLine>,
    pub action_type: ActionType,
    pub exit_dialogue: bool,
}

pub fn enter_dialogue(npc_id: &str, state: &mut WorldState) -> DialogueResult {
    let npc = match state.npcs.get(npc_id) {
        Some(n) => n.clone(),
        None => {
            return DialogueResult {
                messages: vec![OutputLine {
                    text: "There's no one to talk to.".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "NPC not found".into(),
                },
                exit_dialogue: true,
            }
        }
    };

    if npc.hostile {
        return DialogueResult {
            messages: vec![OutputLine {
                text: crate::engine::templates::describe_npc_dialogue(&npc),
                line_type: LineType::Dialogue,
            }],
            action_type: ActionType::NpcDialogue {
                npc_name: npc.name.clone(),
                dialogue_text: "hostile".into(),
            },
            exit_dialogue: true,
        };
    }

    if npc.dialogue_state == DialogueState::Dead {
        return DialogueResult {
            messages: vec![OutputLine {
                text: crate::engine::templates::describe_npc_dialogue(&npc),
                line_type: LineType::Dialogue,
            }],
            action_type: ActionType::NpcDialogue {
                npc_name: npc.name.clone(),
                dialogue_text: "dead".into(),
            },
            exit_dialogue: true,
        };
    }

    state.game_mode = GameMode::InDialogue(npc_id.to_string());

    let dialogue_text = crate::engine::templates::describe_npc_dialogue(&npc);
    let mut messages = vec![OutputLine {
        text: dialogue_text.clone(),
        line_type: LineType::Dialogue,
    }];

    // If quest giver in Greeting state, auto-transition to QuestOffered
    if npc.quest_giver.is_some() && npc.dialogue_state == DialogueState::Greeting {
        if let Some(npc_mut) = state.npcs.get_mut(npc_id) {
            npc_mut.dialogue_state = DialogueState::QuestOffered;
        }
        if let Some(quest_id) = &npc.quest_giver {
            if let Some(quest) = state.quests.get(quest_id) {
                messages.push(OutputLine {
                    text: format!("\"{}\"", quest.description),
                    line_type: LineType::Dialogue,
                });
                messages.push(OutputLine {
                    text: "Will you accept? (yes/no, or 'leave' to end conversation)".into(),
                    line_type: LineType::System,
                });
            }
        }
    }

    // If quest active, check if objective is met
    if npc.dialogue_state == DialogueState::QuestActive {
        if let Some(quest_id) = &npc.quest_giver {
            if let Some(quest) = state.quests.get(quest_id) {
                if quest.completed {
                    if let Some(npc_mut) = state.npcs.get_mut(npc_id) {
                        npc_mut.dialogue_state = DialogueState::QuestComplete;
                    }
                    if let Some(updated_npc) = state.npcs.get(npc_id) {
                        messages.push(OutputLine {
                            text: crate::engine::templates::describe_npc_dialogue(updated_npc),
                            line_type: LineType::Dialogue,
                        });
                    }
                }
            }
        }
    }

    DialogueResult {
        messages,
        action_type: ActionType::NpcDialogue {
            npc_name: npc.name,
            dialogue_text,
        },
        exit_dialogue: false,
    }
}

pub fn process_dialogue_input(
    input: &str,
    npc_id: &str,
    state: &mut WorldState,
) -> DialogueResult {
    let input_lower = input.trim().to_lowercase();

    // Check for exit commands
    if matches!(input_lower.as_str(), "leave" | "goodbye" | "bye" | "exit" | "quit") {
        state.game_mode = GameMode::Exploring;
        let npc_name = state
            .npcs
            .get(npc_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();
        return DialogueResult {
            messages: vec![OutputLine {
                text: format!("You end your conversation with {}.", npc_name),
                line_type: LineType::System,
            }],
            action_type: ActionType::DisplayOnly,
            exit_dialogue: true,
        };
    }

    let npc = match state.npcs.get(npc_id) {
        Some(n) => n.clone(),
        None => {
            state.game_mode = GameMode::Exploring;
            return DialogueResult {
                messages: vec![],
                action_type: ActionType::DisplayOnly,
                exit_dialogue: true,
            };
        }
    };

    match npc.dialogue_state {
        DialogueState::QuestOffered => {
            if matches!(input_lower.as_str(), "yes" | "y" | "accept" | "sure" | "ok") {
                // Accept quest
                if let Some(quest_id) = &npc.quest_giver {
                    if let Some(quest) = state.quests.get_mut(quest_id) {
                        quest.active = true;
                    }
                    if let Some(npc_mut) = state.npcs.get_mut(npc_id) {
                        npc_mut.dialogue_state = DialogueState::QuestActive;
                    }
                    let quest_name = state
                        .quests
                        .get(quest_id)
                        .map(|q| q.name.clone())
                        .unwrap_or_default();
                    let mut quest_messages = vec![OutputLine {
                        text: format!("{} nods gratefully.", npc.name),
                        line_type: LineType::Dialogue,
                    }];
                    if let Some(quest) = state.quests.get(quest_id) {
                        quest_messages.push(OutputLine {
                            text: crate::engine::templates::describe_quest_started(quest),
                            line_type: LineType::System,
                        });
                    }
                    return DialogueResult {
                        messages: quest_messages,
                        action_type: ActionType::QuestStarted { quest_name },
                        exit_dialogue: false,
                    };
                }
            } else if matches!(input_lower.as_str(), "no" | "n" | "decline" | "nah") {
                if let Some(npc_mut) = state.npcs.get_mut(npc_id) {
                    npc_mut.dialogue_state = DialogueState::Familiar;
                }
                return DialogueResult {
                    messages: vec![OutputLine {
                        text: format!("{} looks disappointed. \"Perhaps another time.\"", npc.name),
                        line_type: LineType::Dialogue,
                    }],
                    action_type: ActionType::NpcDialogue {
                        npc_name: npc.name,
                        dialogue_text: "declined".into(),
                    },
                    exit_dialogue: false,
                };
            }
        }
        DialogueState::QuestComplete => {
            // Give rewards
            if let Some(quest_id) = &npc.quest_giver {
                if let Some(quest) = state.quests.get(quest_id) {
                    let rewards = quest.reward.clone();
                    let mut dropped_rewards = Vec::new();
                    for item_id in &rewards {
                        if state.player.inventory.len() < state.player.max_inventory {
                            state.player.inventory.push(item_id.clone());
                        } else {
                            dropped_rewards.push(item_id.clone());
                            // Drop to current location
                            if let Some(loc) =
                                state.locations.get_mut(&state.player.location)
                            {
                                loc.items.push(item_id.clone());
                            }
                        }
                    }
                    let quest_name = quest.name.clone();
                    if let Some(npc_mut) = state.npcs.get_mut(npc_id) {
                        npc_mut.dialogue_state = DialogueState::Familiar;
                    }
                    let reward_names: Vec<String> = rewards
                        .iter()
                        .filter_map(|id| state.items.get(id).map(|i| i.name.clone()))
                        .collect();
                    let completion_text = state
                        .quests
                        .get(quest_id)
                        .map(crate::engine::templates::describe_quest_completed)
                        .unwrap_or_else(|| "Quest complete!".to_string());
                    let mut messages = vec![OutputLine {
                        text: completion_text,
                        line_type: LineType::System,
                    }];
                    if !reward_names.is_empty() {
                        messages.push(OutputLine {
                            text: format!("You received: {}", reward_names.join(", ")),
                            line_type: LineType::System,
                        });
                    }
                    if !dropped_rewards.is_empty() {
                        let dropped_names: Vec<String> = dropped_rewards
                            .iter()
                            .filter_map(|id| state.items.get(id).map(|i| i.name.clone()))
                            .collect();
                        if !dropped_names.is_empty() {
                            messages.push(OutputLine {
                                text: format!(
                                    "Inventory full! {} dropped to the ground.",
                                    dropped_names.join(", ")
                                ),
                                line_type: LineType::System,
                            });
                        }
                    }
                    return DialogueResult {
                        messages,
                        action_type: ActionType::QuestCompleted { quest_name },
                        exit_dialogue: false,
                    };
                }
            }
        }
        _ => {}
    }

    // Generic response for unhandled dialogue
    DialogueResult {
        messages: vec![OutputLine {
            text: format!("{} considers your words.", npc.name),
            line_type: LineType::Dialogue,
        }],
        action_type: ActionType::NpcDialogue {
            npc_name: npc.name,
            dialogue_text: input.to_string(),
        },
        exit_dialogue: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_dialogue_state() -> WorldState {
        let mut state = WorldState::default();
        state.locations.insert(
            "courtyard".into(),
            Location {
                id: "courtyard".into(),
                name: "Courtyard".into(),
                description: "A courtyard.".into(),
                items: vec![],
                npcs: vec!["merchant".into()],
                exits: HashMap::new(),
                locked_exits: HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );
        state.npcs.insert(
            "merchant".into(),
            Npc {
                id: "merchant".into(),
                name: "Merchant".into(),
                description: "A ghostly merchant.".into(),
                personality_seed: "formal".into(),
                dialogue_state: DialogueState::Greeting,
                hostile: false,
                health: 1,
                max_health: 1,
                attack: 0,
                defense: 0,
                items: vec![],
                quest_giver: Some("test_quest".into()),
            },
        );
        state.quests.insert(
            "test_quest".into(),
            Quest {
                id: "test_quest".into(),
                name: "Test Quest".into(),
                description: "Find the thing.".into(),
                giver: "merchant".into(),
                objective: QuestObjective::FetchItem("thing".into()),
                reward: vec![],
                completed: false,
                active: false,
            },
        );
        state
    }

    #[test]
    fn enter_dialogue_with_quest_giver() {
        let mut state = make_dialogue_state();
        let result = enter_dialogue("merchant", &mut state);
        assert!(!result.exit_dialogue);
        assert_eq!(state.game_mode, GameMode::InDialogue("merchant".into()));
    }

    #[test]
    fn accept_quest() {
        let mut state = make_dialogue_state();
        enter_dialogue("merchant", &mut state);
        let result = process_dialogue_input("yes", "merchant", &mut state);
        assert!(state.quests.get("test_quest").unwrap().active);
        assert!(matches!(result.action_type, ActionType::QuestStarted { .. }));
    }

    #[test]
    fn leave_dialogue() {
        let mut state = make_dialogue_state();
        enter_dialogue("merchant", &mut state);
        let result = process_dialogue_input("leave", "merchant", &mut state);
        assert!(result.exit_dialogue);
        assert_eq!(state.game_mode, GameMode::Exploring);
    }

    #[test]
    fn hostile_npc_cant_talk() {
        let mut state = make_dialogue_state();
        state.npcs.get_mut("merchant").unwrap().hostile = true;
        let result = enter_dialogue("merchant", &mut state);
        assert!(result.exit_dialogue);
    }
}
