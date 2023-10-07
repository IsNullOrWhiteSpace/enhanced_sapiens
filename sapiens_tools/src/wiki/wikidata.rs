use std::fmt::Debug;

use mediawiki::api::Api;
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};
use serde_json;

/// A Tool to query Wikidata using SPARQL.
///
/// Wikidata is a free and open knowledge base that can be read and edited by
/// both humans and machines.
///
/// Wikidata acts as central storage for the structured data of its Wikimedia
/// sister projects including Wikipedia, Wikivoyage, Wiktionary, Wikisource, and
/// others.
#[derive(ProtoToolInvoke, ProtoToolDescribe)]
#[tool(
    name = "Wikidata",
    input = "WikidataToolInput",
    output = "WikidataToolOutput"
)]
pub struct WikidataTool {
    client: Api,
}

impl Debug for WikidataTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WikidataTool").finish()
    }
}

/// [`WikidataTool`] input
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct WikidataToolInput {
    /// SPARQL query to execute.
    query: String,
}

/// [`WikidataTool`] output
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct WikidataToolOutput {
    /// SPARQL query result - in JSON.
    result: String,
}

impl WikidataTool {
    /// Create a new [`WikidataTool`]
    pub async fn new() -> Self {
       