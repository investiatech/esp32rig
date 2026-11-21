pub trait Tool: Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> &'static str;
    fn execute(&self, arguments: &[u8], output: &mut [u8]) -> Result<usize, ToolError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolError {
    InvalidArguments,
    BufferTooSmall,
    ExecutionFailed,
}
