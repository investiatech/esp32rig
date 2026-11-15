#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<'a> {
 pub role: Role,
    pub content: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(role: Role, content: &'a str) -> Self {
        Self { role, content }
    }
}