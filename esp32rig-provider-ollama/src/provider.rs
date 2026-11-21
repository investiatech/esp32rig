use crate::config::OllamaConfig;

pub struct OllamaProvider {
    config: OllamaConfig,
}

impl OllamaProvider {
    pub const fn new(config: OllamaConfig) -> Self {
        Self { config }
    }

    pub const fn config(&self) -> &OllamaConfig {
        &self.config
    }
}
