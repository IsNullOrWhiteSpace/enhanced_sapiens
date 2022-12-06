use sapiens::tools::toolbox::Stats;
use serde::{Deserialize, Serialize};

use crate::traces::{CompletionStatus, Event, Trace, Usage};
use crate::Config;

/// The score of a trial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    /// The number of attempted tool invocations - Conclude Tool is not counted
    /// here.
    attempted_invocations: u32,
    /// The number of successful tool invocations - Conclude Tool is not counted
    /// here.
    successful_invocations: u32,
    /// The number of tokens
    tokens: Usage,
    /// Completion status
    completed: bool,
    /// Reached accepting state
    reached_accepting_state: bool,
    /// Termination message
    termination_message: Option<String>,
    /// Tool utilization statistics
    tool_stats: Stats,
    /// The final state name
    final_state_name: String,
}

/// A trial is a task execution with a given configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trial {
    /// The trace of the execution
    trace: Trace,
    /// The task
    task: String,
    /// The configuration,
    config: Config,
    /// The Analysis of the run
 