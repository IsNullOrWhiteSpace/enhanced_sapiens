use std::fmt::Debug;

use reqwest::{Client, Response, Url};
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::error;

use crate::search::gce::SearchResultNumber::Four;
use crate::search::gce::{ErrorBody, Item, Lr, QueryParameters, SearchResults};

/// A Tool to search the web - powered by Google Custom Search Engine.
#[cfg(feature = "search")]
pub mod gce;

/// A Tool to search the web - powered by Google Custom Search Engine.
///
/// Returns a list of [`Item`] with: `title`, `link`, `snippet`.
#[derive(ProtoToolInvoke, ProtoToolDescribe)]
#[tool(
    name = "Search",
    input = "SearchToolInput",
    output = "SearchToolOutput"
)]
pub struct SearchTool {
    /// API key to use
    api_key: String,
    /// CSE ID to use
    cse_id: String,
    /// HTTP client
    client: Mutex<Client>,
}

impl Debug for SearchTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchTool").finish()
    }
}

/// [`SearchTool`] input
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct SearchToolInput {
    /// query to search. `q` parameter of the Google Custom Search Engine API
    /// Use `exclude_terms` and `exact_terms` to refine your search.
    q: String,

    /// word or phrase t