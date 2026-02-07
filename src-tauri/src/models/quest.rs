use serde::{Deserialize, Serialize};

pub type QuestId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuestObjective {
    FetchItem(String),
    KillNpc(String),
    ReachLocation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub id: QuestId,
    pub name: String,
    pub description: String,
    pub giver: String,
    pub objective: QuestObjective,
    pub reward: Vec<String>,
    pub completed: bool,
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_serde_roundtrip() {
        let quest = Quest {
            id: "rats_request".into(),
            name: "The Rat's Request".into(),
            description: "Find cheese for Gristle.".into(),
            giver: "gristle_rat".into(),
            objective: QuestObjective::FetchItem("cellar_cheese".into()),
            reward: vec!["health_potion".into()],
            completed: false,
            active: false,
        };
        let json = serde_json::to_string(&quest).unwrap();
        assert!(json.contains("fetchItem"));
        let deserialized: Quest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "rats_request");
        assert!(!deserialized.completed);
    }
}
