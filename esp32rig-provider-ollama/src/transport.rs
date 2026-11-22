#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    ConnectionFailed,
    Timeout,
    BufferTooSmall,
    InvalidResponse,
    UnsupportedUrl,
}

#[derive(Debug, Clone, Copy)]
pub struct TransportResponse {
    pub status: u16,
    pub body_len: usize,
}

pub trait HttpTransport {
    async fn post(
        &mut self,
        host: &str,
        port: u16,
        path: &str,
        body: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<TransportResponse, TransportError>;
}

