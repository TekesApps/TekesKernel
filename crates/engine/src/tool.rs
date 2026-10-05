use crate::ToolBackend;

/// Runs the Slice-4 write-ahead tool pipeline through the one ToolBackend
/// seam. The engine owns ordering; the backend owns only the external effect.
pub fn execute_tool<B: ToolBackend>(
    pipeline: &mut tools::ToolPipeline<'_>,
    execution: &tools::ToolExecution,
    hooks: &[tools::HookBinding],
    backend: &mut B,
) -> Result<tools::PipelineDecision, tools::ToolPipelineError> {
    pipeline.execute_terminal(execution, hooks, |execution, invocation| {
        backend.execute(execution, invocation)
    })
}
