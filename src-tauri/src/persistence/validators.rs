use std::collections::BTreeMap;

use serde_json::Value;

pub const SAVE_SLOT_MAX_LEN: usize = 32;
pub const THEME_NAME_MAX_LEN: usize = 40;
pub const THEME_KEYS: [&str; 15] = [
    "--bg",
    "--text",
    "--text-dim",
    "--text-bright",
    "--accent",
    "--error",
    "--combat",
    "--dialogue",
    "--input",
    "--system",
    "--border",
    "--panel-bg",
    "--hp-high",
    "--hp-mid",
    "--hp-low",
];

fn normalize_user_label(value: &str, field_name: &str, max_len: usize) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{field_name} cannot be empty."));
    }
    if trimmed.len() > max_len {
        return Err(format!(
            "{field_name} must be {max_len} characters or fewer."
        ));
    }
    if trimmed.chars().any(|c| c.is_control()) {
        return Err(format!("{field_name} cannot contain control characters."));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_'))
    {
        return Err(format!(
            "{field_name} can only use letters, numbers, spaces, '-' and '_'."
        ));
    }
    Ok(trimmed.to_string())
}

pub fn validate_save_slot_name(value: &str) -> Result<String, String> {
    normalize_user_label(value, "Save name", SAVE_SLOT_MAX_LEN)
}

pub fn validate_theme_name(value: &str) -> Result<String, String> {
    normalize_user_label(value, "Theme name", THEME_NAME_MAX_LEN)
}

fn is_valid_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

pub fn validate_theme_config(config: &str) -> Result<BTreeMap<String, String>, String> {
    let parsed: Value =
        serde_json::from_str(config).map_err(|_| "Theme config must be valid JSON.".to_string())?;
    let object = parsed
        .as_object()
        .ok_or_else(|| "Theme config must be a JSON object.".to_string())?;

    let mut sanitized = BTreeMap::new();
    for key in THEME_KEYS {
        let Some(value) = object.get(key) else {
            return Err(format!("Theme config is missing '{key}'."));
        };
        let Some(color) = value.as_str() else {
            return Err(format!("Theme value for '{key}' must be a string."));
        };
        if !is_valid_hex_color(color) {
            return Err(format!(
                "Theme value for '{key}' must be a hex color like #A1B2C3."
            ));
        }
        sanitized.insert(key.to_string(), color.to_string());
    }

    for key in object.keys() {
        if !THEME_KEYS.contains(&key.as_str()) {
            return Err(format!("Theme config contains unsupported key '{key}'."));
        }
    }

    Ok(sanitized)
}

#[cfg(test)]
mod tests {
    use super::{validate_save_slot_name, validate_theme_config, validate_theme_name};

    #[test]
    fn save_slot_name_is_trimmed_and_validated() {
        assert_eq!(
            validate_save_slot_name("  My Save  ").unwrap(),
            "My Save".to_string()
        );
        assert!(validate_save_slot_name("bad/save").is_err());
        assert!(validate_save_slot_name("   ").is_err());
    }

    #[test]
    fn theme_name_is_trimmed_and_validated() {
        assert_eq!(
            validate_theme_name("  Amber Night  ").unwrap(),
            "Amber Night".to_string()
        );
        assert!(validate_theme_name("Theme:One").is_err());
    }

    #[test]
    fn theme_config_validates_strict_shape() {
        let config = serde_json::json!({
            "--bg": "#0A0A0A",
            "--text": "#33FF33",
            "--text-dim": "#1A8C1A",
            "--text-bright": "#66FF66",
            "--accent": "#00CC00",
            "--error": "#FF4444",
            "--combat": "#FF6666",
            "--dialogue": "#66CCCC",
            "--input": "#FFAA33",
            "--system": "#888888",
            "--border": "#1A3A1A",
            "--panel-bg": "#0D0D0D",
            "--hp-high": "#33FF33",
            "--hp-mid": "#FFCC00",
            "--hp-low": "#FF4444"
        })
        .to_string();

        let validated = validate_theme_config(&config).unwrap();
        assert_eq!(validated["--bg"], "#0A0A0A");
        assert_eq!(validated.len(), 15);
    }

    #[test]
    fn theme_config_rejects_unknown_keys() {
        let result = validate_theme_config(
            &serde_json::json!({
                "--bg": "#0A0A0A",
                "--text": "#33FF33",
                "--text-dim": "#1A8C1A",
                "--text-bright": "#66FF66",
                "--accent": "#00CC00",
                "--error": "#FF4444",
                "--combat": "#FF6666",
                "--dialogue": "#66CCCC",
                "--input": "#FFAA33",
                "--system": "#888888",
                "--border": "#1A3A1A",
                "--panel-bg": "#0D0D0D",
                "--hp-high": "#33FF33",
                "--hp-mid": "#FFCC00",
                "--hp-low": "#FF4444",
                "--bad": "#FFFFFF"
            })
            .to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unsupported key"));
    }

    #[test]
    fn theme_config_rejects_invalid_colors() {
        let config = serde_json::json!({
            "--bg": "red",
            "--text": "#33FF33",
            "--text-dim": "#1A8C1A",
            "--text-bright": "#66FF66",
            "--accent": "#00CC00",
            "--error": "#FF4444",
            "--combat": "#FF6666",
            "--dialogue": "#66CCCC",
            "--input": "#FFAA33",
            "--system": "#888888",
            "--border": "#1A3A1A",
            "--panel-bg": "#0D0D0D",
            "--hp-high": "#33FF33",
            "--hp-mid": "#FFCC00",
            "--hp-low": "#FF4444"
        })
        .to_string();

        let result = validate_theme_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("hex color"));
    }
}
