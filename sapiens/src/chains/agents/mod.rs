/// OODA agents
pub mod ooda;

use crate::chains::Outcome;
use crate::context;
use crate::prompt::Task;
use crate::tools::ToolUseError;

/// Error from the agent
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Failed to add to the chat history
    #[error("Failed to add to the chat history: {0}")]
    ChatHistoryError(#[from] context::Error),
    /// Error from the model
    #[error("Error from the model: {0}")]
    ModelError(#[from] crate::models::Error),
}

/// Format the outcome of a task
pub(crate) fn format_outcome(
    task: &Task,
    invocation_count: &usize,
    tool_name: &Option<String>,
    outcome: &Outcome,
) -> String {
    match outcome {
        Outcome::Success { result } => {
          