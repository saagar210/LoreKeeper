use serde::{Deserialize, Serialize};

pub type ItemId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Key,
    Scroll,
    Quest,
    Miscellaneous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatModifier {
    pub attack: i32,
    pub defense: i32,
    pub health: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub modifier: Option<StatModifier>,
    pub usable: bool,
    pub consumable: bool,
    pub key_id: Option<String>,
    #[serde(default)]
    pub lore: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_serde_roundtrip() {
        let item = Item {
            id: "short_sword".into(),
            name: "Short Sword".into(),
            description: "A well-balanced blade.".into(),
            item_type: ItemType::Weapon,
            modifier: Some(StatModifier {
                attack: 3,
                defense: 0,
                health: 0,
            }),
            usable: false,
            consumable: false,
            key_id: None,
            lore: None,
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("itemType"));
        assert!(json.contains("keyId"));
        let deserialized: Item = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "short_sword");
        assert_eq!(deserialized.modifier.unwrap().attack, 3);
    }

    #[test]
    fn stat_modifier_default() {
        let m = StatModifier::default();
        assert_eq!(m.attack, 0);
        assert_eq!(m.defense, 0);
        assert_eq!(m.health, 0);
    }
}
