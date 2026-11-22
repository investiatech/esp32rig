use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OllamaChatRequest<'a> {
    pub model: &'a str,
    pub body: &'a [OllamaMessage<'a>],
    pub stream: bool,
}

#[derive(Debug, Serialize)]
pub struct OllamaMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}
