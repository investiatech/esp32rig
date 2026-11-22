use crate::client::OllamaClient;
use crate::config::OllamaConfig;
use crate::transport::HttpTransport;

pub struct OllamaProvider<T> {
    client: OllamaClient,
    transport: T,
}

impl<T: HttpTransport> OllamaProvider<T> {
    pub fn new(config: OllamaConfig, transport: T) -> Self {
        Self {
            client: OllamaClient::new(config),
            transport,
        }
    }

    pub const fn config(&self) -> &OllamaClient {
        &self.client
    }

    pub fn transport(&mut self) -> &mut T {
        &mut self.transport
    }
}
