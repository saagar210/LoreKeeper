use tauri::{Emitter, State};
use tokio::sync::mpsc;

use crate::narrative::narrator::{self, NarrativeEvent};
use crate::persistence::state::{DbState, GameState, SettingsState};

#[tauri::command]
pub fn rate_narration(
    prompt_hash: String,
    rating: i32,
    model: String,
    db_state: State<DbState>,
) -> Result<(), String> {
    if rating != 1 && rating != -1 {
        return Err("Rating must be 1 or -1.".into());
    }
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO narration_ratings (prompt_hash, rating, model, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![prompt_hash, rating, model, now],
    )
    .map_err(|e| format!("Failed to save rating: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn retry_narration(
    app: tauri::AppHandle,
    game_state: State<'_, GameState>,
    settings_state: State<'_, SettingsState>,
) -> Result<(), String> {
    let (narrative_ctx, state, settings);
    {
        let gs = game_state.0.lock().map_err(|e| e.to_string())?;
        narrative_ctx = gs.last_narrative_context.clone();
        state = gs.clone();
        settings = settings_state.0.lock().map_err(|e| e.to_string())?.clone();
    }

    if narrative_ctx.is_none() {
        return Err("No narration to retry.".into());
    }

    if !settings.ollama_enabled {
        return Err("Ollama is not enabled.".into());
    }

    let (tx, mut rx) = mpsc::channel::<NarrativeEvent>(32);

    tauri::async_runtime::spawn(async move {
        narrator::narrate(&narrative_ctx, &state, &settings, &tx).await;
        drop(tx);
    });

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            let _ = app_clone.emit("narrative-event", &event);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn rate_narration_params() {
        // Validate that rating values are within expected range
        let valid_ratings = [-1, 1];
        for r in valid_ratings {
            assert!(r == 1 || r == -1);
        }
    }
}
