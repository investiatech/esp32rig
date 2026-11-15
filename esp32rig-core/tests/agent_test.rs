use esp32rig_core::model::{ChatRequest, ChatResponse, ToolCall};
use esp32rig_core::prelude::*;

struct TestModel;

impl Model for TestModel {
    fn chat<'a>(
        &mut self,
        _request: &ChatRequest<'_>,
        _response_buffor: &'a mut [u8],
        _tool_calls: &'a mut [ToolCall<'a>],
    ) -> Result<ChatResponse<'a>, ModelError> {
        Ok(ChatResponse::new(None, &[]))
    }
}

struct LedTool;

impl Tool for LedTool {
    fn name(&self) -> &'static str {
        "turn_on_led"
    }

    fn description(&self) -> &'static str {
        "Turns on the LED"
    }

    fn parameters_schema(&self) -> &'static str {
        "{}"
    }

    fn execute(&self, _arguments: &[u8], output: &mut [u8]) -> Result<usize, ToolError> {
        let result = b"LED turned on";

        if output.len() < result.len() {
            return Err(ToolError::BufferTooSmall);
        }

        output[..result.len()].copy_from_slice(result);

        Ok(result.len())
    }
}

static LED_TOOL: LedTool = LedTool;

static TOOLS: [&'static dyn Tool; 1] = [&LED_TOOL];

#[test]
fn agent_executes_led_tool() {
    let registry = ToolRegistry::new(&TOOLS);

    let agent = Agent::new(TestModel, registry);

    let mut output = [0u8; 64];

    let result = agent.execute_tool("turn_on_led", b"{}", &mut output);

    assert!(result.is_ok());

    let length = result.unwrap();

    assert_eq!(&output[..length], b"LED turned on");
}
