use crate::tool::Tool;

pub struct ToolRegistry {
    tools: &'static [&'static dyn Tool],
}

impl ToolRegistry {
    pub const fn new(tools: &'static [&'static dyn Tool]) -> Self {
        Self { tools }
    }

    pub fn find(&self, name: &str) -> Option<&'static dyn Tool> {
        self.tools.iter().copied().find(|tool| tool.name() == name)
    }

    pub fn tools(&self) -> &'static [&'static dyn Tool] {
        self.tools
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
    
}
