
//! Execution chains
//! - [x] OODA - Observe, Orient, Decide, Act
//!   - [x] in single step - See [`SingleStepOODAChain`]
//!   - [x] in several steps - See [`MultiStepOODAChain`]
//! - [ ] 2205.11916 - Zeroshot reasoners - "Let's think step by step" - 2022
//! - [ ] 2207.05608 - Inner monologue - Different types of feedbacks - 2022
//! - [ ] 2302.00083 - In context RALM - Jan 2023
//! - [ ] 2302.01560 - DEPS - Describe, explain, plan, select stages. Feb 2023
//! - [ ] 2210.03629 - ReAct - Reasoning + Action - Mar 2023
//! - [ ] 2303.11366 - Reflexion - heuristic + self-reflection - Mar 2023
//! - [ ] 2303.17071 - DERA - Distinct roles+responsibilities - Mar 2023
//! - [ ] 2305.10601 - Tree of Thoughts - May 2023
// TODO(ssoudan) + LLM self-consistency

// FUTURE(ssoudan) more chains

/// Agents
pub mod agents;
/// Schedulers are responsible for deciding which agent to run next.
pub mod schedulers;

#[cfg(test)]
mod tests;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::chains::agents::ooda::{multistep, one_step};
use crate::chains::schedulers::{MultiAgentScheduler, SingleAgentScheduler};
use crate::context::ContextDump;
use crate::models::Usage;
use crate::tools::invocation::InvocationError;
use crate::tools::toolbox::{invoke_tool, InvokeResult, Toolbox};
use crate::tools::{TerminationMessage, ToolUseError};
use crate::{SapiensConfig, WeakRuntimeObserver};

/// Outcome of an invocation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Outcome {
    /// The invocation was successful
    Success {
        /// The result of the invocation
        result: String,
    },
    /// No valid invocation was found
    NoValidInvocationsFound {
        /// The invocation error
        e: InvocationError,
    },
    /// No invocation was found
    NoInvocationsFound {
        /// The invocation error
        e: InvocationError,
    },
    /// The invocation failed
    ToolUseError {
        /// The tool use error
        e: ToolUseError,
    },
}

/// A message that can be produced by an agent for another agent