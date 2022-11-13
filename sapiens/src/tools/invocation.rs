
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::tools::ToolInvocationInput;

/// Error while extracting tool invocations
#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum InvocationError {
    /// Invalid yaml
    #[error("Invalid yaml: {0}")]
    InvalidYaml(String),
    /// No invocation found in the document
    #[error("No Action found")]
    NoInvocationFound,
    /// No valid invocation found in the document
    #[error("No valid Action found: {0}")]
    NoValidInvocationFound(String),
    /// Too many yaml blocks
    #[error("Too many ({0}) yaml blocks. Only one is expected.")]
    TooManyYamlBlocks(usize),
}

/// One of several T
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Invocation<T> {
    Single(T),
    Multiple(Vec<T>),
}

/// extract on or several T from a string.
///
/// `data` can be a single yaml of T or of a Vec<T> or a list of yaml
/// documents separated by `---`.
fn extract_from_yaml<T>(data: &str) -> Result<Vec<T>, InvocationError>
where
    T: DeserializeOwned,
{
    let mut invocations = vec![];

    for doc in serde_yaml::Deserializer::from_str(data) {
        // is it a list of T or a single T?
        let attempt: Result<Invocation<T>, _> = Deserialize::deserialize(doc);
        match attempt {
            Ok(invocation) => match invocation {
                Invocation::Single(t) => {
                    invocations.push(t);
                }
                Invocation::Multiple(ts) => {
                    invocations.extend(ts);
                }
            },
            Err(e) => {
                debug!(error = %e, "Failed to deserialize as a list of T or a single T");
                return Err(InvocationError::InvalidYaml(e.to_string()));
            }
        }
    }

    if invocations.is_empty() {
        Err(InvocationError::NoInvocationFound)
    } else {
        Ok(invocations)
    }
}

/// Extracted invocations
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ExtractedInvocations {
    pub(crate) invocations: Vec<ToolInvocationInput>,
    pub(crate) yaml_block_count: usize,
}

/// Find all the invocations in a markdown document.
pub(crate) fn find_all(data: &str) -> Result<ExtractedInvocations, InvocationError> {
    let mut err: Option<InvocationError> = None;

    let mut invocations = vec![];
    let mut yaml_block_count = 0;

    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        // we have start of a yaml block
        if line.trim().starts_with("```yaml") {
            // collect the lines until the end of the yaml block
            let mut yaml = vec![];

            for line in lines.by_ref() {
                if line.trim().starts_with("```") {
                    break;
                }

                yaml.push(line);
            }

            // put them together
            let yaml = yaml.join("\n");

            yaml_block_count += 1;

            // does that make valid invocations?
            match extract_from_yaml(&yaml) {
                Ok(more) => {
                    invocations.extend(more);
                }
                Err(e) => {
                    // debug!(error = %e, "Failed to extract invocation from yaml");

                    err = Some(e);
                }
            }
        }
    }

    if invocations.is_empty() {
        if let Some(err) = err {
            Err(err)
        } else {
            Err(InvocationError::NoInvocationFound)
        }
    } else {
        Ok(ExtractedInvocations {
            invocations,
            yaml_block_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use insta::{assert_display_snapshot, assert_yaml_snapshot};
    use serde_yaml::Number;

    #[tokio::test]
    async fn test_find_invocations_one_yaml() {
        let data = indoc! {r#"# Some text
    ```yaml
    tool_name: Search
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    ```        
    Some other text
    "#};

        let invocations = super::find_all(data).unwrap();

        assert_eq!(invocations.invocations.len(), 1);
    }

    #[tokio::test]
    async fn test_find_multiple_invocations() {
        let data = indoc! {r#"# Some text
    ```yaml
    tool_name: Search1
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    ---
    tool_name: Search2
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    ```        
    Some other text
    ```yaml
    tool_name: Search3
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    something: else
    ```        
    Some other text
    ```yaml
    - tool_name: Search4
      parameters:
        q: Marcel Deneuve
        excluded_terms: Resident Evil
        num_results: 10
    - tool_name: Search5
      parameters:
        q: Marcel Deneuve
        excluded_terms: Resident Evil
        num_results: 10
    ```                
    Some other text
    "#};

        let invocations = super::find_all(data).unwrap();

        assert_eq!(invocations.invocations.len(), 5);

        assert_yaml_snapshot!(invocations);
    }

    #[tokio::test]
    async fn test_extraction_of_one_yaml() {
        let data = indoc! {r#"# Some text
    ```yaml
    tool_name: Search
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    ```        
    Some other text
    "#};

        let tool_invocations = super::find_all(data).unwrap();

        assert_eq!(tool_invocations.invocations.len(), 1);

        let invocation = &tool_invocations.invocations[0];

        assert_eq!(invocation.tool_name, "Search");
    }

    #[tokio::test]
    async fn test_extraction_of_one_yaml_with_output() {
        let data = indoc! {r#"# Some text
    ```yaml
    tool_name: Search
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    output: 
      something: | 
        Marcel Deneuve is a character in the Resident Evil film series, playing a minor role in Resident Evil: Apocalypse and a much larger role in Resident Evil: Extinction. Explore historical records and family tree profiles about Marcel Deneuve on MyHeritage, the world's largest family network.
    ```        
    Some other text
    "#};

        let tool_invocations = super::find_all(data).unwrap();

        assert_eq!(tool_invocations.invocations.len(), 1);

        let invocation = &tool_invocations.invocations[0];

        assert_eq!(invocation.tool_name, "Search");
        assert_eq!(invocation.parameters.get("q").unwrap(), "Marcel Deneuve");
        assert_eq!(
            invocation.parameters.get("excluded_terms").unwrap(),
            "Resident Evil"
        );
        assert_eq!(
            invocation.parameters.get("num_results").unwrap(),
            &serde_yaml::Value::Number(Number::from(10))
        );
        assert!(!invocation.junk.is_empty());
        assert!(invocation.junk.get("output").is_some());
    }

    #[tokio::test]
    async fn test_extraction_of_three_yaml_with_output() {
        let data = indoc! {r#"# Some text
    ```yaml
    tool_name: Search1
    parameters:
      q: Marcel Deneuve
      excluded_terms: Resident Evil
      num_results: 10
    output: 
      something: | 
        Marcel Deneuve is a character in the Resident Evil film series, playing a minor role in Resident Evil: Apocalypse and a much larger role in Resident Evil: Extinction. Explore historical records and family tree profiles about Marcel Deneuve on MyHeritage, the world's largest family network.
    ```        
    Some other text
    ```yaml
    tool_name: Search2
    parameters:
      q: Marcel Prouse
      excluded_terms: La Recherche du Temps Perdu
      num_results: 10
    ```        
    Some other other text
    ```yaml
    tool_name: Search3
    parameters:
      q: Marcel et son Orchestre
      excluded_terms: Les Vaches
      num_results: 10
    ```
    That's all folks!          
    "#};

        let tool_invocations = super::find_all(data).unwrap();

        assert_eq!(tool_invocations.invocations.len(), 3);

        let invocation = &tool_invocations.invocations[0];
        assert_eq!(invocation.tool_name, "Search1");