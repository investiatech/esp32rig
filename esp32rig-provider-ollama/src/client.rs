use crate::config::OllamaConfig;
use crate::request::{OllamaChatRequest, OllamaMessage};
use crate::response::OllamaChatResponse;
use crate::transport::{HttpTransport, TransportError, TransportResponse};

pub struct OllamaClient {
    config: OllamaConfig,
}

impl OllamaClient {
    pub fn new(config: OllamaConfig) -> Self {
        Self { config }
    }

    pub const fn config(&self) -> &OllamaConfig {
        &self.config
    }

    pub fn endpoint(&self) -> Result<(&str, u16, &str), TransportError> {
        Self::parse_url(self.config.base_url)
    }

    fn parse_url(url: &str) -> Result<(&str, u16, &str), TransportError> {
        let url = url
            .strip_prefix("http://")
            .ok_or(TransportError::UnsupportedUrl)?;

        let path_start = url.find('/').unwrap_or(url.len());

        let authority = &url[..path_start];
        let path = if path_start == url.len() {
            "/"
        } else {
            &url[path_start..]
        };

        let (host, port) = match authority.rsplit_once(':') {
            Some((host, port)) => {
                let port = port
                    .parse::<u16>()
                    .map_err(|_| TransportError::UnsupportedUrl)?;

                (host, port)
            }
            None => (authority, 80),
        };

        if host.is_empty() {
            return Err(TransportError::UnsupportedUrl);
        }

        Ok((host, port, path))
    }

    pub async fn post_chat(
        &self,
        transport: &mut impl HttpTransport,
        request_body: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<TransportResponse, TransportError> {
        let (host, port, base_path) = Self::parse_url(self.config.base_url)?;

        let path = if base_path == "/" {
            "/api/chat"
        } else {
            return Err(TransportError::UnsupportedUrl);
        };

        transport
            .post(host, port, path, request_body, response_buffer)
            .await
    }

    pub fn serialize_chat_request(
        &self,
        messages: &[OllamaMessage],
    ) -> Result<Vec<u8>, serde_json::Error> {
        let request = OllamaChatRequest {
            model: self.config.model,
            body: messages,
            stream: false,
        };

        serde_json::to_vec(&request)
    }
    pub fn deserialize_chat_response(
        &self,
        response_buffer: &[u8],
        body_len: usize,
    ) -> Result<OllamaChatResponse, TransportError> {
        let body = response_buffer
            .get(..body_len)
            .ok_or(TransportError::InvalidResponse)?;

        serde_json::from_slice(body).map_err(|_| TransportError::InvalidResponse)
    }

    pub async fn chat(
        &self,
        transport: &mut impl HttpTransport,
        message: &[OllamaMessage<'_>],
        response_buffer: &mut [u8],
    ) -> Result<OllamaChatResponse, TransportError> {
        let request_body = self
            .serialize_chat_request(message)
            .map_err(|_| TransportError::InvalidResponse)?;
        let response = self.post_chat(transport, &request_body, response_buffer)
            .await?;

        if response.status != 200 {
            return Err(TransportError::InvalidResponse);
        }

        self.deserialize_chat_response(response_buffer, response.body_len)

    }
}
