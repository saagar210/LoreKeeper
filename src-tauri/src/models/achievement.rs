use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
}

pub fn all_achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_blood".into(),
            name: "First Blood".into(),
            description: "Defeat your first enemy.".into(),
            icon: "sword".into(),
        },
        Achievement {
            id: "explorer".into(),
            name: "Explorer".into(),
            description: "Visit 8 different locations.".into(),
            icon: "compass".into(),
        },
        Achievement {
            id: "completionist".into(),
            name: "Completionist".into(),
            description: "Complete all quests.".into(),
            icon: "scroll".into(),
        },
        Achievement {
            id: "speedrunner".into(),
            name: "Speedrunner".into(),
            description: "Reach the Final Sanctum in under 30 turns.".into(),
            icon: "clock".into(),
        },
        Achievement {
            id: "hoarder".into(),
            name: "Hoarder".into(),
            description: "Have 8 or more items in your inventory at once.".into(),
            icon: "bag".into(),
        },
        Achievement {
            id: "secret_keeper".into(),
            name: "Secret Keeper".into(),
            description: "Discover 3 or more secret commands.".into(),
            icon: "key".into(),
        },
        Achievement {
            id: "survivor".into(),
            name: "Survivor".into(),
            description: "Survive with 5 or fewer HP.".into(),
            icon: "heart".into(),
        },
        Achievement {
            id: "diplomat".into(),
            name: "Diplomat".into(),
            description: "Complete the game through negotiation.".into(),
            icon: "handshake".into(),
        },
        // NEW ACHIEVEMENTS - Phase 2 Content Expansion
        Achievement {
            id: "master_crafter".into(),
            name: "Master Crafter".into(),
            description: "Learn and craft all available recipes.".into(),
            icon: "hammer".into(),
        },
        Achievement {
            id: "vault_hunter".into(),
            name: "Vault Hunter".into(),
            description: "Discover the hidden vault in the Great Hall.".into(),
            icon: "treasure".into(),
        },
        Achievement {
            id: "heart_seeker".into(),
            name: "Heart Seeker".into(),
            description: "Obtain a shard of the Dungeon Heart.".into(),
            icon: "crystal".into(),
        },
        Achievement {
            id: "legendary_collector".into(),
            name: "Legendary Collector".into(),
            description: "Equip a legendary item (Ethereal Blade or Mithril Mail).".into(),
            icon: "crown".into(),
        },
        Achievement {
            id: "peacekeeper".into(),
            name: "Peacekeeper".into(),
            description: "Complete the game through negotiation without excessive violence.".into(),
            icon: "dove".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_achievements_have_unique_ids() {
        let achievements = all_achievements();
        let mut ids: Vec<&str> = achievements.iter().map(|a| a.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), achievements.len());
    }
}
