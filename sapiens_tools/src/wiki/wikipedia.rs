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
/// around the world and hosted by the Wikimedia Foundation.
#[derive(ProtoToolInvoke, ProtoToolDescribe)]
#[tool(
    name = "Wikipedia",
    input = "WikipediaToolInput",
    output = "WikipediaToolOutput"
)]
pub struct WikipediaTool {
    client: Api,
}

impl Debug for WikipediaTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WikipediaTool").finish()
    }
}

/// [`WikipediaTool`] input
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct WikipediaToolInput {
    /// query parameters. E.g.
    /// ```yaml
    ///   parameters:
    ///     action: query
    ///     prop:
    ///       - extracts
    ///       - exintro
    ///       - explaintext
    ///     titles: Albert Einstein
    /// ```
    /// - Values can be either strings or numbers. Or lists of them.
    /// - The result size is limited. Be specific and use limits where possible.
    parameters: HashMap<String, Value>,
    /// maximum number of results to return - if not specified, all results are
    /// returned.
    limit: