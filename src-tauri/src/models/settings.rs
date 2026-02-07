use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeName {
    GreenTerminal,
    AmberTerminal,
    Parchment,
    DarkModern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    pub ollama_enabled: bool,
    pub ollama_model: String,
    pub ollama_url: String,
    pub temperature: f64,
    pub narrator_tone: String,
    pub typewriter_speed: u32,
    pub theme: ThemeName,
    #[serde(default = "default_narration_verbosity")]
    pub narration_verbosity: String,
    #[serde(default)]
    pub sound_enabled: bool,
    #[serde(default = "default_sound_volume")]
    pub sound_volume: f64,
    #[serde(default = "default_difficulty")]
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

fn default_sound_volume() -> f64 {
    0.5
}

fn default_difficulty() -> Difficulty {
    Difficulty::Normal
}

fn default_narration_verbosity() -> String {
    "normal".to_string()
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            ollama_enabled: false,
            ollama_model: "llama3.2".to_string(),
            ollama_url: "http://localhost:11434".to_string(),
            temperature: 0.7,
            narrator_tone: "atmospheric".to_string(),
            typewriter_speed: 30,
            theme: ThemeName::GreenTerminal,
            narration_verbosity: "normal".to_string(),
            sound_enabled: false,
            sound_volume: 0.5,
            difficulty: Difficulty::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default() {
        let s = GameSettings::default();
        assert!(!s.ollama_enabled);
        assert_eq!(s.ollama_url, "http://localhost:11434");
        assert_eq!(s.theme, ThemeName::GreenTerminal);
        assert_eq!(s.typewriter_speed, 30);
    }

    #[test]
    fn settings_serde_roundtrip() {
        let s = GameSettings::default();
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("ollamaEnabled"));
        assert!(json.contains("ollamaModel"));
        assert!(json.contains("ollamaUrl"));
        assert!(json.contains("narratorTone"));
        assert!(json.contains("typewriterSpeed"));
        let deserialized: GameSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ollama_model, "llama3.2");
        assert!((deserialized.temperature - 0.7).abs() < f64::EPSILON);
    }
}
