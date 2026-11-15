use crate::model::ToolCall;

#[derive(Debug, Clone, Copy)]
pub struct ChatResponse<'a> {
    pub content: Option<&'a str>,
    pub tool_calls: &'a [ToolCall<'a>],
}

impl<'a> ChatResponse<'a> {
    pub const fn new(content: Option<&'a str>, tool_calls: &'a [ToolCall<'a>]) -> Self {
        ChatResponse {
            content,
            tool_calls,
        }
    }

    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls.is_empty()
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }
}
