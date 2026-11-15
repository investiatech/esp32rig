use crate::prelude::ToolRegistry;
use crate::tool::ToolError;

pub struct Executor<'a> {
    registry: &'a ToolRegistry,
}

impl<'a> Executor<'a> {
    pub fn new(registry: &'a ToolRegistry) -> Self {
        Self { registry }
    }

    pub fn execute(&self, name: &str, arguments: &[u8], output: &mut [u8]) -> Result<usize, ToolError> {
        let tool = self.registry.find(name).ok_or(ToolError::ExecutionFailed)?;
        tool.execute(arguments, output)
    }

}
