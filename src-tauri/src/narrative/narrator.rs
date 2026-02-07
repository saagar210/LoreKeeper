use futures::StreamExt;

use crate::models::*;
use crate::narrative::context;
use crate::narrative::ollama::OllamaClient;
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

    let messages = context::build_narrative_messages(context, &settings_with_tone);

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

#[allow(clippy::too_many_arguments)]
pub async fn narrate_dialogue(
    npc_name: &str,
    personality_seed: &str,
    dialogue_text: &str,
    settings: &GameSettings,
    relationship: i32,
    memory: &[NpcMemory],
    dialogue_history: &[(String, String)],
    sender: &tokio::sync::mpsc::Sender<NarrativeEvent>,
) {
    if !settings.ollama_enabled {
        let _ = sender.send(NarrativeEvent::Fallback).await;
        return;
    }

    let messages = context::build_dialogue_messages(
        npc_name,
        personality_seed,
        dialogue_text,
        settings,
        relationship,
        memory,
        dialogue_history,
    );

    let client = OllamaClient::new(&settings.ollama_url);

    match client
        .chat_stream(messages, &settings.ollama_model, settings.temperature)
        .await
    {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut got_any = false;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn narrate_dialogue_sends_fallback_when_ollama_disabled() {
        let settings = GameSettings::default(); // ollama_enabled is false by default
        let (tx, mut rx) = tokio::sync::mpsc::channel::<NarrativeEvent>(8);

        narrate_dialogue(
            "Merchant",
            "formal",
            "Hello",
            &settings,
            0,
            &[],
            &[],
            &tx,
        )
        .await;
        drop(tx);

        let event = rx.recv().await;
        assert!(matches!(event, Some(NarrativeEvent::Fallback)));
        // No more events
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn narrate_sends_fallback_when_ollama_disabled() {
        let settings = GameSettings::default();
        let state = WorldState::default();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<NarrativeEvent>(8);

        narrate(&None, &state, &settings, &tx).await;
        drop(tx);

        let event = rx.recv().await;
        assert!(matches!(event, Some(NarrativeEvent::Fallback)));
    }
}
