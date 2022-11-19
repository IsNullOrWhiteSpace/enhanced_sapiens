
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::tools;
use crate::tools::invocation::InvocationError;
use crate::tools::{
    AdvancedTool, TerminalTool, TerminationMessage, Tool, ToolDescription, ToolUseError,
};