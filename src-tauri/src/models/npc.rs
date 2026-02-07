use serde::{Deserialize, Serialize};

pub type NpcId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DialogueState {
    Greeting,
    Familiar,
    QuestOffered,
    QuestActive,
    QuestComplete,
    Hostile,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub id: NpcId,
    pub name: String,
    pub description: String,
    pub personality_seed: String,
    pub dialogue_state: DialogueState,
    pub hostile: bool,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub items: Vec<String>,
    pub quest_giver: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npc_serde_roundtrip() {
        let npc = Npc {
            id: "merchant_ghost".into(),
            name: "The Dead Merchant".into(),
            description: "A translucent figure.".into(),
            personality_seed: "Melancholic, formal".into(),
            dialogue_state: DialogueState::Greeting,
            hostile: false,
            health: 1,
            max_health: 1,
            attack: 0,
            defense: 0,
            items: vec![],
            quest_giver: Some("merchants_unfinished_business".into()),
        };
        let json = serde_json::to_string(&npc).unwrap();
        assert!(json.contains("personalitySeed"));
        assert!(json.contains("dialogueState"));
        assert!(json.contains("questGiver"));
        assert!(json.contains("maxHealth"));
        let deserialized: Npc = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "merchant_ghost");
        assert_eq!(deserialized.dialogue_state, DialogueState::Greeting);
    }
}
