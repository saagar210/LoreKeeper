use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JournalCategory {
    Lore,
    Bestiary,
    Location,
    Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalEntry {
    pub id: String,
    pub category: JournalCategory,
    pub title: String,
    pub content: String,
    pub discovered_turn: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn journal_entry_serde_roundtrip() {
        let entry = JournalEntry {
            id: "loc_courtyard".into(),
            category: JournalCategory::Location,
            title: "Courtyard".into(),
            content: "A crumbling courtyard.".into(),
            discovered_turn: 3,
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("discoveredTurn"));
        assert!(json.contains("\"category\":\"location\""));
        let deserialized: JournalEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "loc_courtyard");
        assert_eq!(deserialized.category, JournalCategory::Location);
    }

    #[test]
    fn journal_category_serde() {
        let categories = vec![
            (JournalCategory::Lore, "\"lore\""),
            (JournalCategory::Bestiary, "\"bestiary\""),
            (JournalCategory::Location, "\"location\""),
            (JournalCategory::Item, "\"item\""),
        ];
        for (cat, expected) in categories {
            let json = serde_json::to_string(&cat).unwrap();
            assert_eq!(json, expected);
        }
    }
}
