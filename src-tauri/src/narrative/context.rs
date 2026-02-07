use crate::models::*;
use crate::narrative::ollama::ChatMessage;

pub fn build_narrative_messages(
    context: &NarrativeContext,
    settings: &GameSettings,
) -> Vec<ChatMessage> {
    let length_instruction = match settings.narration_verbosity.as_str() {
        "brief" => "1 sentence max per response.",
        "verbose" => "4-6 sentences per response. Be richly descriptive.",
        _ => "2-3 sentences max per response.",
    };

    let system_msg = format!(
        "You are the narrator for a dark fantasy text adventure called \"The Depths of Thornhold.\" \
         Tone: {}. Be atmospheric and concise. {} \
         Never contradict the game state provided. Never mention game mechanics directly. \
         Describe what the player experiences, not what the system does.",
        settings.narrator_tone, length_instruction
    );

    let inventory_str = if context.inventory_names.is_empty() {
        "nothing".to_string()
    } else {
        context.inventory_names.join(", ")
    };

    let room_items_str = if context.room_item_names.is_empty() {
        "nothing".to_string()
    } else {
        context.room_item_names.join(", ")
    };

    let room_npcs_str = if context.room_npc_names.is_empty() {
        "no one".to_string()
    } else {
        context.room_npc_names.join(", ")
    };

    let action_desc = describe_action_type(&context.action_type);

    let user_msg = format!(
        "CURRENT STATE:\n\
         - Location: \"{}\" — {}\n\
         - Player: {}/{} HP, carrying: {}\n\
         - Room contains: {}, with {}\n\
         - Mood: {}\n\
         - Turns elapsed: {}\n\n\
         ACTION: {}\n\n\
         Narrate this moment.",
        context.location_name,
        context.location_description,
        context.player_health,
        context.player_max_health,
        inventory_str,
        room_items_str,
        room_npcs_str,
        context.mood,
        context.turns_elapsed,
        action_desc
    );

    vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_msg,
        },
        ChatMessage {
            role: "user".to_string(),
            content: user_msg,
        },
    ]
}

pub fn build_dialogue_messages(
    npc_name: &str,
    personality_seed: &str,
    dialogue_text: &str,
    settings: &GameSettings,
    relationship: i32,
    memory: &[NpcMemory],
    dialogue_history: &[(String, String)],
) -> Vec<ChatMessage> {
    let _ = settings;
    let relationship_desc = if relationship > 30 {
        "friendly and warm"
    } else if relationship < -30 {
        "hostile and suspicious"
    } else {
        "neutral"
    };
    let memory_str = if memory.is_empty() {
        String::new()
    } else {
        let recent: Vec<&str> = memory.iter().rev().take(5).map(|m| m.event.as_str()).collect();
        format!(" Recent memories: {}.", recent.join(", "))
    };
    let system_msg = format!(
        "You are voicing \"{}\". Personality: {}. Disposition toward the player: {}.{} \
         Respond in character. 1-2 sentences. Stay consistent with the personality.",
        npc_name, personality_seed, relationship_desc, memory_str
    );

    let mut messages = vec![ChatMessage {
        role: "system".to_string(),
        content: system_msg,
    }];

    // Include last 5 dialogue history entries for multi-turn context
    let history_start = dialogue_history.len().saturating_sub(5);
    for (role, text) in &dialogue_history[history_start..] {
        messages.push(ChatMessage {
            role: role.clone(),
            content: text.clone(),
        });
    }

    // Current user message
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: dialogue_text.to_string(),
    });

    messages
}

fn describe_action_type(action_type: &ActionType) -> String {
    match action_type {
        ActionType::RoomEntered { first_visit } => {
            if *first_visit {
                "Player entered a new room for the first time.".to_string()
            } else {
                "Player returned to a previously visited room.".to_string()
            }
        }
        ActionType::ItemTaken { item_name } => {
            format!("Player picked up {}.", item_name)
        }
        ActionType::ItemDropped { item_name } => {
            format!("Player dropped {}.", item_name)
        }
        ActionType::ItemUsed { item_name, effect } => {
            format!("Player used {}. Effect: {}", item_name, effect)
        }
        ActionType::ItemEquipped { item_name } => {
            format!("Player equipped {}.", item_name)
        }
        ActionType::ItemUnequipped { item_name } => {
            format!("Player unequipped {}.", item_name)
        }
        ActionType::CombatAttack {
            damage,
            target_name,
            target_hp,
            target_max_hp,
        } => {
            format!(
                "Player attacked {} for {} damage. Target HP: {}/{}",
                target_name, damage, target_hp, target_max_hp
            )
        }
        ActionType::CombatDefend {
            damage,
            attacker_name,
        } => {
            format!("{} attacked player for {} damage.", attacker_name, damage)
        }
        ActionType::CombatVictory { enemy_name } => {
            format!("Player defeated {}.", enemy_name)
        }
        ActionType::CombatFlee { success } => {
            if *success {
                "Player successfully fled from combat.".to_string()
            } else {
                "Player failed to flee from combat.".to_string()
            }
        }
        ActionType::PlayerDeath => "Player has died.".to_string(),
        ActionType::NpcDialogue {
            npc_name,
            dialogue_text,
        } => {
            format!("{} said: {}", npc_name, dialogue_text)
        }
        ActionType::QuestStarted { quest_name } => {
            format!("Quest started: {}", quest_name)
        }
        ActionType::QuestCompleted { quest_name } => {
            format!("Quest completed: {}", quest_name)
        }
        ActionType::EventTriggered { event_description } => {
            format!("Event: {}", event_description)
        }
        ActionType::DisplayOnly => "Information displayed.".to_string(),
        ActionType::Error { message } => {
            format!("Error: {}", message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_dialogue_messages_no_history() {
        let settings = GameSettings::default();
        let msgs = build_dialogue_messages(
            "Merchant",
            "formal and melancholic",
            "Hello there",
            &settings,
            0,
            &[],
            &[],
        );
        // system + user = 2 messages
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, "system");
        assert!(msgs[0].content.contains("Merchant"));
        assert!(msgs[0].content.contains("formal and melancholic"));
        assert!(msgs[0].content.contains("neutral"));
        assert_eq!(msgs[1].role, "user");
        assert_eq!(msgs[1].content, "Hello there");
    }

    #[test]
    fn build_dialogue_messages_with_history() {
        let settings = GameSettings::default();
        let history = vec![
            ("user".to_string(), "What do you sell?".to_string()),
            ("assistant".to_string(), "I sell fine wares.".to_string()),
        ];
        let msgs = build_dialogue_messages(
            "Merchant",
            "formal",
            "How much for the sword?",
            &settings,
            0,
            &[],
            &history,
        );
        // system + 2 history + current user = 4 messages
        assert_eq!(msgs.len(), 4);
        assert_eq!(msgs[0].role, "system");
        assert_eq!(msgs[1].role, "user");
        assert_eq!(msgs[1].content, "What do you sell?");
        assert_eq!(msgs[2].role, "assistant");
        assert_eq!(msgs[2].content, "I sell fine wares.");
        assert_eq!(msgs[3].role, "user");
        assert_eq!(msgs[3].content, "How much for the sword?");
    }

    #[test]
    fn build_dialogue_messages_caps_history_at_5() {
        let settings = GameSettings::default();
        let history: Vec<(String, String)> = (0..8)
            .map(|i| ("user".to_string(), format!("msg {}", i)))
            .collect();
        let msgs = build_dialogue_messages(
            "Guard",
            "stern",
            "current",
            &settings,
            0,
            &[],
            &history,
        );
        // system + 5 history + current user = 7 messages
        assert_eq!(msgs.len(), 7);
        // First history entry should be msg 3 (index 3), not msg 0
        assert_eq!(msgs[1].content, "msg 3");
        assert_eq!(msgs[5].content, "msg 7");
        assert_eq!(msgs[6].content, "current");
    }

    #[test]
    fn build_dialogue_messages_relationship_friendly() {
        let settings = GameSettings::default();
        let msgs = build_dialogue_messages(
            "Friend",
            "kind",
            "hi",
            &settings,
            50,
            &[],
            &[],
        );
        assert!(msgs[0].content.contains("friendly and warm"));
    }

    #[test]
    fn build_dialogue_messages_relationship_hostile() {
        let settings = GameSettings::default();
        let msgs = build_dialogue_messages(
            "Enemy",
            "gruff",
            "hi",
            &settings,
            -50,
            &[],
            &[],
        );
        assert!(msgs[0].content.contains("hostile and suspicious"));
    }

    #[test]
    fn build_dialogue_messages_with_memory() {
        let settings = GameSettings::default();
        let memory = vec![
            NpcMemory { turn: 1, event: "talked".to_string() },
            NpcMemory { turn: 2, event: "quest_accepted".to_string() },
        ];
        let msgs = build_dialogue_messages(
            "Merchant",
            "formal",
            "hi",
            &settings,
            0,
            &memory,
            &[],
        );
        assert!(msgs[0].content.contains("quest_accepted"));
        assert!(msgs[0].content.contains("talked"));
    }
}
