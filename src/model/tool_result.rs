#[derive(Debug, Clone, Copy)]
pub struct ToolResult<'a> {
    pub tool_call_id: &'a str,
    pub name: &'a str,
    pub content: &'a str,
}

impl<'a> ToolResult<'a> {
    pub fn new(tool_call_id: &'a str, name: &'a str, content: &'a str) -> Self {
        Self {
            tool_call_id,
            name,
            content,
        }
    }
}
