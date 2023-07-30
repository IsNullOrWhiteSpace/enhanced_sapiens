use std::fmt::Display;

use arxiv;
use arxiv::{Arxiv, ArxivQuery};
use sapiens::tools::{Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError};
use sapiens_derive::{Describe, ProtoToolDescribe, ProtoToolInvoke};
use serde::{Deserialize, Serialize};

/// A Tool to query arXiv.
///
/// arXiv is a free distribution service and an open-access archive for
/// millions scholarly articles in the fields of physics, mathematics, computer
/// science, quantitative biology, quantitative finance, statistics, electrical
/// engineering and systems science, and economics. Materials on this site are
/// not peer-reviewed by arXiv.
#[derive(Debug, ProtoToolInvoke, ProtoToolDescribe)]
#[tool(name = "Arxiv", input = "ArxivToolInput", output = "ArxivToolOutput")]
pub struct ArxivTool {}

/// Sort order
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum SortOrder {
    /// Ascending
    #[serde(rename = "ascending")]
    Ascending,
    /// Descending
    #[serde(rename = "descending")]
    #[default]
    Descending,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "ascending"),
            SortOrder::Descending => write!(f, "descending"),
        }
    }
}

/// Sort by
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum SortBy {
    /// Relevance
    #[serde(rename = "relevance")]
    #[default]
    Relevance,
    /// Last updated date
    #[serde(rename = "lastUpdatedDate")]
    LastUpdatedDate,
    /// Submitted date
    #[serde(rename = "submittedDate")]
    SubmittedDate,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Relevance => write!(f, "relevance"),
            SortBy::LastUpdatedDate => write!(f, "lastUpdatedDate"),
            SortBy::SubmittedDate => write!(f, "submittedDate"),
        }
    }
}

/// [`ArxivTool`] input
///
/// Arxiv API documentation query specification
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct ArxivToolInput {
    /// search_query: Search query - see https://info.arxiv.org/help/api/user-manual.html
    /// for details. E.g. `cs.AI` or `cat:cs.AI` or `au:John Smith`
    /// The fields that can be searched are: `ti` (title), `au` (author), `abs`
    /// (abstract), `co` (comment), `jr` (journal reference), `cat` (subject
    /// category), `rn` (report number), `id` (id (use id_list instead)),
    /// `all` (all of the above). Operators: `AND`, `OR`, `ANDNOT`.
    /// You cannot search on publication or last update date.
    pub search_query: String,

    /// id_list: Comma-separated list of arXiv IDs to return
    pub id_list: Option<String>,

    /// start: Result offset for pagination
    pub start: Option<i32>,

    /// max_results: Maximum number of results to return in a single response.
    /// Default is 10. Maximum allowed value is 100.
    pub max_results: Option<i32>,

    /// Sort by. Can be either `relevance`, `lastUpdatedDate` or
    /// `submittedDate`. Default is `relevance`.
    pub sort_by: Option<SortBy>,

    /// Sort order. Can be either `ascending` or `descending`.
    /// Default is `descending`.
    pub sort_order: Option<SortOrder>,

    /// True to gather PDF url - default is false
    pub show_pdf_url: Option<bool>,

    /// True to gather authors - default is false
    pub show_authors: Option<bool>,

    /// True to gather comments - default is false
    pub show_comments: Option<bool>,

    /// True to gather summary - default is false
    pub show_summary: Option<bool>,
}

impl From<&ArxivToolInput> for ArxivQuery {
    fn from(input: &ArxivToolInput) -> Self {
        ArxivQuery {
            base_url: "https://export.arxiv.org/api/query?".to_string(),
            search_query: input.search_query.clone(),
            id_list: input.id_list.clone().unwrap_or_default(),
            start: input.start,
            max_results: input.max_results,
            sort_by: input.sort_by.clone().unwrap_or_default().to_string(),
            sort_order: input.sort_order.clone().unwrap_or_default().to_string(),
        }
    }
}

/// [`ArxivTool`] output
#[derive(Debug, Deserialize, Serialize, Describe)]
pub struct ArxivToolOutput {
    // FUTURE(ssoudan) proc_macro_derive to generate this
    /// query result. `ArxivResult` is an object containing the following
    /// fields:
    /// - `id`: <str> arXiv ID
    /// - `updated`: <str> last updated date
    /// - `published