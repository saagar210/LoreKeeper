use std::net::IpAddr;

use reqwest::Url;
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

pub fn normalize_ollama_url(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Ollama URL cannot be empty.".into());
    }

    let mut parsed = Url::parse(trimmed)
        .map_err(|_| "Ollama URL must be a valid http://localhost address.".to_string())?;

    if parsed.scheme() != "http" {
        return Err("Ollama URL must use http.".into());
    }

    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("Ollama URL cannot include credentials.".into());
    }

    if parsed.query().is_some() || parsed.fragment().is_some() {
        return Err("Ollama URL cannot include query strings or fragments.".into());
    }

    if parsed.path() != "/" && !parsed.path().is_empty() {
        return Err("Ollama URL must point to the server root.".into());
    }

    let host = parsed
        .host_str()
        .ok_or_else(|| "Ollama URL must include a host.".to_string())?;
    let is_loopback = host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<IpAddr>()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false);

    if !is_loopback {
        return Err("Ollama URL must point to localhost or a loopback address.".into());
    }

    parsed
        .set_path("");
    Ok(parsed.to_string().trim_end_matches('/').to_string())
}

impl GameSettings {
    pub fn validated_for_update(mut self) -> Result<Self, String> {
        self.ollama_url = normalize_ollama_url(&self.ollama_url)?;
        self.temperature = self.temperature.clamp(0.0, 2.0);
        self.sound_volume = self.sound_volume.clamp(0.0, 1.0);
        Ok(self)
    }

    pub fn sanitize_loaded(mut self) -> Self {
        self.temperature = self.temperature.clamp(0.0, 2.0);
        self.sound_volume = self.sound_volume.clamp(0.0, 1.0);

        match normalize_ollama_url(&self.ollama_url) {
            Ok(url) => self.ollama_url = url,
            Err(_) => {
                self.ollama_enabled = false;
                self.ollama_url = Self::default().ollama_url;
            }
        }

        self
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

    #[test]
    fn normalizes_loopback_ollama_urls() {
        assert_eq!(
            normalize_ollama_url(" http://localhost:11434/ ").unwrap(),
            "http://localhost:11434"
        );
        assert_eq!(
            normalize_ollama_url("http://127.0.0.1:11434").unwrap(),
            "http://127.0.0.1:11434"
        );
        assert_eq!(
            normalize_ollama_url("http://[::1]:11434/").unwrap(),
            "http://[::1]:11434"
        );
    }

    #[test]
    fn rejects_non_loopback_ollama_urls() {
        assert!(normalize_ollama_url("https://localhost:11434").is_err());
        assert!(normalize_ollama_url("http://192.168.1.10:11434").is_err());
        assert!(normalize_ollama_url("http://example.com:11434").is_err());
        assert!(normalize_ollama_url("http://localhost:11434/api").is_err());
    }

    #[test]
    fn sanitize_loaded_disables_invalid_ollama_settings() {
        let sanitized = GameSettings {
            ollama_enabled: true,
            ollama_url: "http://example.com:11434".into(),
            temperature: 8.0,
            sound_volume: 2.0,
            ..GameSettings::default()
        }
        .sanitize_loaded();

        assert!(!sanitized.ollama_enabled);
        assert_eq!(sanitized.ollama_url, "http://localhost:11434");
        assert_eq!(sanitized.temperature, 2.0);
        assert_eq!(sanitized.sound_volume, 1.0);
    }
}
