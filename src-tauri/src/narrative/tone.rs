use crate::models::*;

pub fn determine_tone(state: &WorldState) -> String {
    if state.player.max_health <= 0 {
        return "ominous, foreboding, something is deeply wrong".to_string();
    }
    let health_pct =
        (state.player.health as f64 / state.player.max_health as f64 * 100.0) as i32;

    let location_mood = state
        .locations
        .get(&state.player.location)
        .map(|l| l.ambient_mood)
        .unwrap_or(Mood::Peaceful);

    // Combat just ended (check if any quest was just completed)
    let recent_victory = matches!(state.game_mode, GameMode::Exploring)
        && state.quests.values().any(|q| q.completed);

    if health_pct < 25 {
        return "desperate, visceral, every breath feels like the last".to_string();
    }

    if matches!(state.game_mode, GameMode::InCombat(_)) {
        return "tense, urgent, danger at every moment".to_string();
    }

    if recent_victory && health_pct > 75 {
        return "triumphant with dry humor, hard-won relief".to_string();
    }

    match location_mood {
        Mood::Peaceful => {
            if health_pct > 75 {
                "calm, reflective, atmospheric".to_string()
            } else {
                "uneasy calm, a moment to catch breath".to_string()
            }
        }
        Mood::Tense => "tense, watchful, danger lurks nearby".to_string(),
        Mood::Mysterious => "curious, atmospheric, secrets in the shadows".to_string(),
        Mood::Dark => "ominous, foreboding, darkness presses close".to_string(),
        Mood::Sacred => "reverent, mysterious, ancient power lingers".to_string(),
        Mood::Dangerous => "perilous, every step could be the last".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_changes_with_health() {
        let mut state = WorldState::default();
        state.locations.insert(
            "courtyard".into(),
            Location {
                id: "courtyard".into(),
                name: "Courtyard".into(),
                description: "Test.".into(),
                items: vec![],
                npcs: vec![],
                exits: std::collections::HashMap::new(),
                locked_exits: std::collections::HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );

        state.player.health = 100;
        let tone = determine_tone(&state);
        assert!(tone.contains("calm"));

        state.player.health = 10;
        let tone = determine_tone(&state);
        assert!(tone.contains("desperate"));
    }

    #[test]
    fn tone_in_combat() {
        let mut state = WorldState::default();
        state.game_mode = GameMode::InCombat("enemy".into());
        let tone = determine_tone(&state);
        assert!(tone.contains("tense"));
    }
}
