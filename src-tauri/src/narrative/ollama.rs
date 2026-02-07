use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub name: String,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OllamaStatus {
    pub connected: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    options: ChatOptions,
}

#[derive(Debug, Serialize)]
struct ChatOptions {
    temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatStreamResponse {
    message: Option<ChatStreamMessage>,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct ChatStreamMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    models: Vec<TagsModel>,
}

#[derive(Debug, Deserialize)]
struct TagsModel {
    name: String,
    size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    version: String,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn check_health(&self) -> Result<OllamaStatus, String> {
        let url = format!("{}/api/version", self.base_url);
        match self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(version_resp) = resp.json::<VersionResponse>().await {
                        Ok(OllamaStatus {
                            connected: true,
                            version: Some(version_resp.version),
                        })
                    } else {
                        Ok(OllamaStatus {
                            connected: true,
                            version: None,
                        })
                    }
                } else {
                    Ok(OllamaStatus {
                        connected: false,
                        version: None,
                    })
                }
            }
            Err(_) => Ok(OllamaStatus {
                connected: false,
                version: None,
            }),
        }
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, String> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

        let tags: TagsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse models: {}", e))?;

        Ok(tags
            .models
            .into_iter()
            .map(|m| ModelInfo {
                name: m.name,
                size: m.size,
            })
            .collect())
    }

    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        temperature: f64,
    ) -> Result<impl futures::Stream<Item = Result<String, String>>, String> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: true,
            options: ChatOptions { temperature },
        };

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Ollama returned status: {}", resp.status()));
        }

        // Use a buffered approach to handle NDJSON lines split across chunks
        let stream = futures::stream::unfold(
            (resp.bytes_stream(), String::new()),
            |(mut byte_stream, mut buffer)| async move {
                loop {
                    // Try to extract a complete line from the buffer
                    if let Some(newline_pos) = buffer.find('\n') {
                        let line = buffer[..newline_pos].trim().to_string();
                        buffer = buffer[newline_pos + 1..].to_string();

                        if line.is_empty() {
                            continue;
                        }

                        if let Ok(parsed) = serde_json::from_str::<ChatStreamResponse>(&line) {
                            if parsed.done {
                                return None; // Stream complete
                            }
                            if let Some(msg) = parsed.message {
                                if !msg.content.is_empty() {
                                    return Some((Ok(msg.content), (byte_stream, buffer)));
                                }
                            }
                        }
                        continue;
                    }

                    // Need more data
                    match byte_stream.next().await {
                        Some(Ok(bytes)) => {
                            buffer.push_str(&String::from_utf8_lossy(&bytes));
                        }
                        Some(Err(e)) => {
                            return Some((
                                Err(format!("Stream error: {}", e)),
                                (byte_stream, buffer),
                            ));
                        }
                        None => {
                            // Stream ended; process any remaining buffer
                            let line = buffer.trim().to_string();
                            if !line.is_empty() {
                                if let Ok(parsed) =
                                    serde_json::from_str::<ChatStreamResponse>(&line)
                                {
                                    if !parsed.done {
                                        if let Some(msg) = parsed.message {
                                            if !msg.content.is_empty() {
                                                return Some((
                                                    Ok(msg.content),
                                                    (byte_stream, String::new()),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            return None;
                        }
                    }
                }
            },
        );

        Ok(stream)
    }
}
