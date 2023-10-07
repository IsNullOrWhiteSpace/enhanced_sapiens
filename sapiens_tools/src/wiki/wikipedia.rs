use std::collections::HashMap;
use std::fmt::Debug;

use mediawiki::api::Api;
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml::Value;

/// A Tool to query Wikipedia using SPARQL.
///
/// Wikipedia is a free online encyclopedia, created and edited by volunteers
/// around the