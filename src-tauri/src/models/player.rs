use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatusEffectType {
    Poison,
    Blessed,
    Weakened,
    Burning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub name: String,
    pub turns_remaining: i32,
    pub damage_per_turn: i32,
    pub attack_modifier: i32,
    pub defense_modifier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub location: String,
    pub inventory: Vec<String>,
    pub max_inventory: usize,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub equipped_weapon: Option<String>,
    pub equipped_armor: Option<String>,
    pub quest_flags: HashMap<String, bool>,
    pub visited_locations: HashSet<String>,
    pub turns_elapsed: u32,
    #[serde(default)]
    pub status_effects: Vec<StatusEffect>,
    #[serde(default)]
    pub discovered_secrets: Vec<String>,
}

impl Default for Player {
    fn default() -> Self {
        let mut visited = HashSet::new();
        visited.insert("courtyard".to_string());
        Self {
            location: "courtyard".to_string(),
            inventory: Vec::new(),
            max_inventory: 10,
            health: 100,
            max_health: 100,
            attack: 5,
            defense: 3,
            equipped_weapon: None,
            equipped_armor: None,
            quest_flags: HashMap::new(),
            visited_locations: visited,
            turns_elapsed: 0,
            status_effects: Vec::new(),
            discovered_secrets: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_default() {
        let p = Player::default();
        assert_eq!(p.location, "courtyard");
        assert_eq!(p.health, 100);
        assert_eq!(p.max_health, 100);
        assert_eq!(p.attack, 5);
        assert_eq!(p.defense, 3);
        assert_eq!(p.max_inventory, 10);
        assert!(p.visited_locations.contains("courtyard"));
    }

    #[test]
    fn player_serde_roundtrip() {
        let p = Player::default();
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("maxInventory"));
        assert!(json.contains("maxHealth"));
        assert!(json.contains("equippedWeapon"));
        assert!(json.contains("visitedLocations"));
        assert!(json.contains("turnsElapsed"));
        let deserialized: Player = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.location, "courtyard");
        assert_eq!(deserialized.health, 100);
    }
}
