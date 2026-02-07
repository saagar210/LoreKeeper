use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LocationId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "north" | "n" => Some(Self::North),
            "south" | "s" => Some(Self::South),
            "east" | "e" => Some(Self::East),
            "west" | "w" => Some(Self::West),
            "up" | "u" => Some(Self::Up),
            "down" | "d" => Some(Self::Down),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::North => "North",
            Self::South => "South",
            Self::East => "East",
            Self::West => "West",
            Self::Up => "Up",
            Self::Down => "Down",
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Mood {
    Peaceful,
    Tense,
    Mysterious,
    Dark,
    Sacred,
    Dangerous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub exits: HashMap<Direction, LocationId>,
    pub locked_exits: HashMap<Direction, String>,
    pub visited: bool,
    pub discovered_secrets: Vec<String>,
    pub ambient_mood: Mood,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_parse() {
        assert_eq!(Direction::parse("north"), Some(Direction::North));
        assert_eq!(Direction::parse("n"), Some(Direction::North));
        assert_eq!(Direction::parse("south"), Some(Direction::South));
        assert_eq!(Direction::parse("s"), Some(Direction::South));
        assert_eq!(Direction::parse("invalid"), None);
    }

    #[test]
    fn direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::Up.opposite(), Direction::Down);
    }

    #[test]
    fn location_serde_roundtrip() {
        let loc = Location {
            id: "courtyard".into(),
            name: "The Courtyard".into(),
            description: "A stone courtyard.".into(),
            items: vec!["rusty_lantern".into()],
            npcs: vec!["merchant_ghost".into()],
            exits: HashMap::from([(Direction::East, "great_hall".into())]),
            locked_exits: HashMap::new(),
            visited: false,
            discovered_secrets: vec![],
            ambient_mood: Mood::Peaceful,
        };
        let json = serde_json::to_string(&loc).unwrap();
        assert!(json.contains("ambientMood"));
        assert!(json.contains("discoveredSecrets"));
        let deserialized: Location = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "courtyard");
        assert_eq!(deserialized.name, "The Courtyard");
    }
}
