#[derive(Debug, Clone, Copy)]
pub struct ToolCall<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub arguments: &'a str,
}

impl<'a> ToolCall<'a> {
    pub fn new(id: &'a str, name: &'a str, arguments: &'a str) -> Self {
        Self {
            id,
            name,
            arguments,
        }
    }
}
