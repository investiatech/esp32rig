use crate::executor::Executor;
use crate::model::{ChatRequest, ChatResponse, Message, Model, ModelError, ToolCall};
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

    pub async fn chat<'a>(
        &mut self,
        messages: &'a [Message<'a>],
        response_buffer: &'a mut [u8],
        tool_call_buffer: &'a mut [ToolCall<'a>],
    ) -> Result<ChatResponse<'a>, ModelError> {
        let request = ChatRequest {
            messages,
            tools: self.tools.tools(),
        };

        self.model
            .chat(&request, response_buffer, tool_call_buffer)
            .await
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
