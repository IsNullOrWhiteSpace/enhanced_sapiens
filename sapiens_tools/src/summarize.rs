use std::fmt::Debug;

use async_openai::config::OpenAIConfig;
use async_openai::types::{CreateCompletionRequest, Prompt};
use async_openai::Client;
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};

/// Text summarization tool
#[derive(ProtoToolDescribe, ProtoToolInvoke)]
#[tool(
    name = "Summarize",
    input = "SummarizeToolInput",
    output = "SummarizeToolOutput"
)]
pub struct SummarizeTool {
    openai_client: Client<OpenAIConfig>,
    model: String,
}

impl Debug for SummarizeTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SummarizeTool").finish()
    }
}

impl SummarizeTool {
    /// Create a new SummarizeTool
    pub fn with_model(openai_client: Client<OpenAIConfig>, model: String) -> Self {
        Self {
            openai_client,
            model,
        }
    }

    /// Create a new SummarizeTool with the default model
    pub fn new(openai_client: Client<OpenAIConfig>) -> Self {
        Self::with_model(openai_client, "text-babbage-001".to_string())
    }
}

impl Default for SummarizeTool {
    fn default() -> Self {
        Self {
            openai_client: Client::new(),
            model: "text-babbage-001".to_string(),
        }
    }
}

/// A tool that is called to test stuffs
#[derive(Debug, Serialize, Deserialize, Describe)]
pub struct SummarizeToolInput {
    /// The text to summarize (max 2000 characters)
    pub text: String,
}

/// Summa