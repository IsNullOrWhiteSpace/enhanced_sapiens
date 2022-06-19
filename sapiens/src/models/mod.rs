pub mod openai;
pub mod vertex_ai;

use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::context::ChatEntry;

/// A model reference
pub type ModelRef = Arc<Box<dyn Model>>;

/// Errors from the models
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The openai error
    #[error("Model invocation failed")]
    OpenAIError(#[from] openai::OpenAIError),
    /// No response from the model
    #[error("No response from the model")]
    NoResponseFromModel,
    /// The model is not supported
    #[error("Model not supported: {0}")]
    ModelNotSupported(String),
    /// Vertex AI error
    #[error("Vertex AI error: {0}")]
    VertexAIError(#[from] gcp_vertex_ai_generative_language::Error),
    /// Filtered output
    #[error("Filtered output")]
    Filtered,
}

/// Roles in the conversation
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// The system
    System,
    /// The user
    #[default]
    User,
    /// The assistant
    Assistant,
    /// Function call
    Function,
    /// Tool call
    Tool,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::Function => write!(f, "function"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

// FUTURE(ssoudan) support pure