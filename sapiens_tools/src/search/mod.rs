use std::fmt::Debug;

use reqwest::{Client, Response, Url};
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::error;

use crate::search::gce::SearchResultNumber::Four;
use crate::search::gce::