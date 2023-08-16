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

    /// word or phrase to exclude from search
    exclude_terms: Option<String>,

    /// word or phrase that must be in the search results
    exact_terms: Option<String>,

    /// number of results to return (max 10, default 4)
    num: Option<u32>,

    /// language restriction (default 'lang_en') - see https://developers.google.com/custom-search/v1/reference/rest/v1/cse/list#query-parameters
    lr: Option<String>,

    /// start index (default 1)
    start_index: Option<u32>,
}

/// [`SearchTool`] output
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct SearchToolOutput {
    /// result items. An [`Item`] has the following format: `{'title':
    /// '...', 'link': '...', 'snippet': '...'}`
    results: Vec<Item>,
    /// Next page start index if any
    next_start_index: Option<u32>,
}

impl From<&SearchToolInput> for QueryParameters {
    fn from(value: &SearchToolInput) -> Self {
        let mut q = QueryParameters::builder();

        let q = q
            .q(&value.q)
            .num(value.num.and_then(|x| x.try_into().ok()).unwrap_or(Four))
           