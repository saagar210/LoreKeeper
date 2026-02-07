use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::event::GameEvent;
use super::item::Item;
use super::location::Location;
use super::npc::Npc;
use super::player::Player;
use super::quest::Quest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineType {
    Narration,
    System,
    Error,
    PlayerInput,
    Combat,
    Dialogue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputLine {
    pub text: String,
    pub line_type: LineType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionType {
    RoomEntered { first_visit: bool },
    ItemTaken { item_name: String },
    ItemDropped { item_name: String },
    ItemUsed { item_name: String, effect: String },
    ItemEquipped { item_name: String },
    ItemUnequipped { item_name: String },
    CombatAttack { damage: i32, target_name: String, target_hp: i32, target_max_hp: i32 },
    CombatDefend { damage: i32, attacker_name: String },
    CombatVictory { enemy_name: String },
    CombatFlee { success: bool },
    PlayerDeath,
    NpcDialogue { npc_name: String, dialogue_text: String },
    QuestStarted { quest_name: String },
    QuestCompleted { quest_name: String },
    EventTriggered { event_description: String },
    DisplayOnly,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NarrativeContext {
    pub location_name: String,
    pub location_description: String,
    pub mood: String,
    pub player_health: i32,
    pub player_max_health: i32,
    pub inventory_names: Vec<String>,
    pub room_item_names: Vec<String>,
    pub room_npc_names: Vec<String>,
    pub action_type: ActionType,
    pub turns_elapsed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub messages: Vec<OutputLine>,
    pub action_type: ActionType,
    pub narrative_context: Option<NarrativeContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    pub messages: Vec<OutputLine>,
    pub world_state: WorldState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EndingType {
    VictoryPeace,
    VictoryCombat,
    Death,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameMode {
    Exploring,
    InCombat(String),
    InDialogue(String),
    GameOver(EndingType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatState {
    pub enemy_id: String,
    pub player_turn: bool,
    pub turn_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatLogEntry {
    pub turn: u32,
    pub attacker: String,
    pub defender: String,
    pub damage: i32,
    pub defender_hp_after: i32,
    pub is_player_attack: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldState {
    pub player: Player,
    pub locations: HashMap<String, Location>,
    pub items: HashMap<String, Item>,
    pub npcs: HashMap<String, Npc>,
    pub quests: HashMap<String, Quest>,
    pub events: Vec<GameEvent>,
    pub game_mode: GameMode,
    pub combat_state: Option<CombatState>,
    pub message_log: Vec<String>,
    pub combat_log: Vec<CombatLogEntry>,
    pub last_narrative_context: Option<NarrativeContext>,
    pub initialized: bool,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            player: Player::default(),
            locations: HashMap::new(),
            items: HashMap::new(),
            npcs: HashMap::new(),
            quests: HashMap::new(),
            events: Vec::new(),
            game_mode: GameMode::Exploring,
            combat_state: None,
            message_log: Vec::new(),
            combat_log: Vec::new(),
            last_narrative_context: None,
            initialized: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_state_default() {
        let ws = WorldState::default();
        assert_eq!(ws.game_mode, GameMode::Exploring);
        assert!(!ws.initialized);
        assert!(ws.combat_state.is_none());
        assert!(ws.locations.is_empty());
    }

    #[test]
    fn game_mode_serde_roundtrip() {
        let modes = vec![
            GameMode::Exploring,
            GameMode::InCombat("goblin".into()),
            GameMode::InDialogue("merchant".into()),
            GameMode::GameOver(EndingType::VictoryPeace),
            GameMode::GameOver(EndingType::VictoryCombat),
            GameMode::GameOver(EndingType::Death),
        ];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let deserialized: GameMode = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, mode);
        }
    }

    #[test]
    fn world_state_serde_roundtrip() {
        let ws = WorldState::default();
        let json = serde_json::to_string(&ws).unwrap();
        assert!(json.contains("gameMode"));
        assert!(json.contains("combatState"));
        assert!(json.contains("messageLog"));
        let deserialized: WorldState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.player.location, "courtyard");
        assert!(!deserialized.initialized);
    }
}
