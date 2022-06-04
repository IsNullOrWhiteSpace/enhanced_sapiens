//! Sapiens library
//!
//! *Sapiens uses tools to interact with the world.*
//!
//! An experiment with handing over the tools to the machine.
//!
//! # Overview
//! This library is the core of Sapiens. It contains the logic for the
//! interaction between the user, the language model and the tools.
//!
//! # More information
//! See https://github.com/ssoudan/sapiens/tree/main/sapiens_cli for an example of usage or
//! https://github.com/ssoudan/sapiens/tree/main/sapiens_bot for a Discord bot.
//!
//! https://github.com/ssoudan/sapiens/tree/main/sapiens_exp is a framework to run experiments and collect traces
//! of the interactions between the language model and the tools to accomplish a
//! task.
//!
//! A collection of tools is defined in https://github.com/ssoudan/sapiens/tree/main/sapiens_tools.
pub mod context;

/// Prompt generation logic
pub mod prompt;

/// Toolbox for sapiens
pub mod tools;

/// Language models
pub mod models;

pub mod chains;

use std::fmt::Debug;
use std::str::FromStr;
use std::sync::{Arc, Weak};

use clap::builder::PossibleValue;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::chains::{Chain, Message, MultiStepOODAChain, SingleStepOODAChain};
use crate::context::{ChatEntry, ContextDump};
use crate::models::openai::OpenAI;
use crate::models::{ModelRef, ModelResponse, Role, Usage};
use crate::tools::invocation::InvocationError;
use crate::tools::toolbox::{InvokeResult, Toolbox};
use crate::tools::{TerminationMessage, ToolUseError};

/// The error type for the bot
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Failed to add to the chat history
    #[error("Failed to add to the chat history: {0}")]
    ChatHistoryError(#[from] context::Error),
    /// Model evaluation error
    #[error("Model evaluation error: {0}")]
    ModelEvaluationError(#[from] models::Error),
    /// Reached the maximum number of steps
    #[error("Maximal number of steps reached")]
    MaxStepsReached,
    /// The response is too long
    #[error("The response is too long: {0}")]
    ActionResponseTooLong(String),
    /// Error in the chain
    #[error("Chain error: {0}")]
    ChainError(#[from] chains::Error),
}

/// Type of chain to use
#[derive(Def