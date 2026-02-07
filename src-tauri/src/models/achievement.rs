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
            id: "bookworm".into(),
            name: "Bookworm".into(),
            description: "Examine 5 different items.".into(),
            icon: "book".into(),
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
