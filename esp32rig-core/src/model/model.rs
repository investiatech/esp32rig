use crate::model::{ChatResponse, ToolCall};
use crate::model::request::ChatRequest;

pub trait Model {
    async fn chat<'a>(
        &mut self,
        request: &ChatRequest<'_>,
        response_buffer: &'a mut [u8],
        tool_call_buffer: &'a mut [ToolCall<'a>],
    ) -> Result<ChatResponse<'a>, ModelError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelError {
    RequestFailed,
    ResponseTooLarge,
    InvalidResponse,
}
