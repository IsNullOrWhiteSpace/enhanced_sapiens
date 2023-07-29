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
    /// category), `rn` (report