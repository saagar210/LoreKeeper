use serde::{Deserialize, Serialize};

use super::location::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventTrigger {
    OnEnter,
    OnTake(String),
    OnUse(String),
    OnKill(String),
    OnTurn(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventAction {
    Damage(i32),
    SpawnNpc(String),
    Unlock(Direction),
    Message(String),
    GiveItem(String),
    SetQuestFlag(String),
    ApplyStatus(super::player::StatusEffect),
    RemoveStatus(String),
    ChangeDescription(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEvent {
    pub trigger: EventTrigger,
    pub action: EventAction,
    pub one_shot: bool,
    pub fired: bool,
    pub location_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_serde_roundtrip() {
        let event = GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Damage(5),
            one_shot: false,
            fired: false,
            location_id: "crypt_passage".into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("oneShot"));
        assert!(json.contains("locationId"));
        let deserialized: GameEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.trigger, EventTrigger::OnEnter);
        assert_eq!(deserialized.action, EventAction::Damage(5));
    }

    #[test]
    fn event_trigger_variants() {
        let triggers = vec![
            EventTrigger::OnEnter,
            EventTrigger::OnTake("sword".into()),
            EventTrigger::OnUse("scroll".into()),
            EventTrigger::OnKill("goblin".into()),
        ];
        for trigger in triggers {
            let json = serde_json::to_string(&trigger).unwrap();
            let deserialized: EventTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, trigger);
        }
    }
}
