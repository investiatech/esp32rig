#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub base_url: &'static str,
    pub model: &'static str,
}

impl OllamaConfig {
    pub fn new(base_url: &'static str, model: &'static str) -> Self {
        Self { base_url, model }
    }
}
