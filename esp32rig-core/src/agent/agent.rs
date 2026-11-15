use crate::executor::Executor;
use crate::model::Model;
use crate::registry::ToolRegistry;
use crate::tool::ToolError;

pub struct Agent<ModelType> {
    model: ModelType,
    tools: ToolRegistry,
}

impl<ModelType: Model> Agent<ModelType> {
    pub const fn new(model: ModelType, tools: ToolRegistry) -> Self {
        Self { model, tools }
    }

    pub fn model(&self) -> &ModelType {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut ModelType {
        &mut self.model
    }

    pub fn tools(&self) -> &ToolRegistry {
        &self.tools
    }

    pub fn execute_tool(
        &self,
        name: &str,
        arguments: &[u8],
        output: &mut [u8],
    ) -> Result<usize, ToolError> {
        let executor = Executor::new(&self.tools);
        executor.execute(name, arguments, output)
    }
}
