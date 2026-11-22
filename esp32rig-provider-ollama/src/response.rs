use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OllamaChatResponse {
    pub model: String,
    pub messages: OllamaResponseMessage,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponseMessage {
    pub role: String,
    pub content: String,
}
