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
    output = "SummarizeToolOutpu