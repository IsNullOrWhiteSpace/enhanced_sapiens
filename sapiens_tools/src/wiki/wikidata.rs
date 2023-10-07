use std::fmt::Debug;

use mediawiki::api::Api;
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};
use serde_json;

/// A Tool to query Wikidata using SPARQL.
///
/// Wikidata is a fre