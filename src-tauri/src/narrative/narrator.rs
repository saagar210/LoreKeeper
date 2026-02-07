use futures::StreamExt;

use crate::models::*;
use crate::narrative::context;
use crate::narrative::ollama::{ChatMessage, OllamaClient};
use crate::narrative::tone;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum NarrativeEvent {
    Token { text: String },
    Complete,
    Fallback,
}

pub async fn narrate(
    narrative_context: &Option<NarrativeContext>,
    state: &WorldState,
    settings: &GameSettings,
    sender: &tokio::sync::mpsc::Sender<NarrativeEvent>,
) {
    if !settings.ollama_enabled {
        let _ = sender.send(NarrativeEvent::Fallback).await;
        return;
    }

    let context = match narrative_context {
        Some(ctx) => ctx,
        None => {
            let _ = sender.send(NarrativeEvent::Fallback).await;
            return;
        }
    };

    // Update tone based on current state
    let mut settings_with_tone = settings.clone();
    settings_with_tone.narrator_tone = tone::determine_tone(state);

    let messages: Vec<ChatMessage> = context::build_narrative_messages(context, &settings_with_tone);

    let client = OllamaClient::new(&settings.ollama_url);

    match client
        .chat_stream(messages, &settings.ollama_model, settings.temperature)
        .await
    {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut got_any = false;

            // Timeout after 10 seconds
            let timeout = tokio::time::timeout(std::time::Duration::from_secs(10), async {
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(text) => {
                            if !text.is_empty() {
                                got_any = true;
                                let _ = sender
                                    .send(NarrativeEvent::Token { text })
                                    .await;
                            }
                        }
                        Err(_) => break,
                    }
                }
                got_any
            })
            .await;

            match timeout {
                Ok(true) => {
                    let _ = sender.send(NarrativeEvent::Complete).await;
                }
                _ => {
                    let _ = sender.send(NarrativeEvent::Fallback).await;
                }
            }
        }
        Err(_) => {
            let _ = sender.send(NarrativeEvent::Fallback).await;
        }
    }
}
