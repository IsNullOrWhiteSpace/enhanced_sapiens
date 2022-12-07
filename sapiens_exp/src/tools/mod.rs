
//! Bake the acceptance test for the task in the tools
//! Scenario -> (Recipe, Config)
//!
//! Recipe -> (Task, Acceptance criteria)
//!
//! Acceptance criteria -> Tools
//!
//! run_until_completion: (Task, Config, Tools) -> (Trace, Stats, Acceptance
//! results)
//!
//! evaluate::Trial::analyze: (Trace, Stats, Acceptance results) -> Analysis
//!
//! evaluate::Trial::build: (Trace, State, Acceptance results, Task, Analysis)
//! -> Trial

use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::Arc;

use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// Scenario 0: Preparing a cereal bowl
pub mod scenario_0;

mod scenario_1;

/// The state of a scenario
pub trait State: Send + Sync {
    /// Reset the state to its initial state
    fn reset(&mut self);
