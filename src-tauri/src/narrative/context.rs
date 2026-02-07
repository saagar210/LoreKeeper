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
) -> Vec<ChatMessage> {
    let _ = settings;
    let system_msg = format!(
        "You are voicing \"{}\". Personality: {}. \
         Respond in character. 1-2 sentences. Stay consistent with the personality.",
        npc_name, personality_seed
    );

    vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_msg,
        },
        ChatMessage {
            role: "user".to_string(),
            content: dialogue_text.to_string(),
        },
    ]
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
