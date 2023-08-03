
use std::cmp::Ordering;
use std::collections::HashMap;

use convert_case::{Case, Casing};
use pyo3::indoc::{formatdoc, indoc};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use sapiens::tools::toolbox::{invoke_simple_from_toolbox, Toolbox};
use sapiens::tools::{
    AdvancedTool, Describe, ProtoToolDescribe, ProtoToolInvoke, ToolDescription, ToolUseError,
};
use sapiens_derive::{Describe, ProtoToolDescribe};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tracing::trace;

/// Conversion tools
pub(crate) mod utils;

use crate::python::utils::SimpleToolDescription;

const MAX_OUTPUT_SIZE: usize = 512;

// FUTURE(ssoudan) install pySWIP

/// A tool that runs sandboxed Python code. Use this to transform data.
///
/// - To use another Tool with parameters `input_field_1` and `input_field_2`
///   and result fields `output_field_1` and `output_field_2` use:
/// ```python
/// result = tools.ToolName(input_field_1=..., input_field_2=...)
/// print(result['output_field_1'])
/// print(result['output_field_2'])
/// ```
/// - Only stdout and stderr are captured and made available (limited to 512B
///   total). If the output is larger, use `tools.Conclude` directly from the
///   code.
/// - List available tools with `tools.list()`. And returns a list of
///   `{'name':.., 'description':.., 'parameters':.., 'responses_content':..,
///   }`.
/// - `open`|`exec` are forbidden.
/// - Limited libraries available: urllib3, requests, sympy, numpy,
/// BeautifulSoup4, feedparser, arxiv.
/// - No PIP.
#[derive(Debug, Default, ProtoToolDescribe)]
#[tool(
    name = "SandboxedPython",
    input = "PythonToolInput",
    output = "PythonToolOutput"
)]
pub struct PythonTool {}

/// The input of the Python tool
#[derive(Debug, Serialize, Deserialize, Describe)]
pub struct PythonToolInput {
    /// The Python code to run. MANDATORY
    pub code: String,
}

/// The output of the Python tool
#[derive(Serialize, Deserialize, Describe)]
pub struct PythonToolOutput {
    /// The stdout output of the Python code.
    pub stdout: String,
    /// The stderr output of the Python code.
    pub stderr: String,
}

#[pyclass]
#[derive(Default)]
struct Logging {
    output: String,
}

#[pymethods]
impl Logging {
    fn write(&mut self, data: &str) {
        self.output.push_str(data);
    }
}

#[pyclass(unsendable)]
struct ToolsWrapper {
    toolbox: Toolbox,
    tool_list: Vec<SimpleToolDescription>,
}

impl ToolsWrapper {
    async fn new(toolbox: Toolbox) -> Self {
        let tools = toolbox.describe().await;
        let tool_list = tools
            .into_values()
            .map(SimpleToolDescription::from)
            .collect::<Vec<_>>();

        ToolsWrapper { toolbox, tool_list }
    }
}

#[pymethods]
impl ToolsWrapper {
    // list all tools
    #[pyo3(signature = ())]
    fn list(&self, py: Python<'_>) -> PyResult<PyObject> {
        let tools = self.tool_list.to_object(py);
        Ok(tools)
    }

    // invoke a tool
    #[pyo3(signature = (tool_name, input))]
    fn invoke(
        &self,
        py: Python<'_>,
        tool_name: &str,
        input: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        // convert PyDict to a serde_yaml::Value
        let input = if let Some(input) = input {
            let input: PyObject = input.into();

            utils::to_yaml(py, &input).map_err(|e| {
                pyo3::exceptions::PyException::new_err(format!("Invalid input: {}", e))
            })?
        } else {
            Value::default()
        };

        let (tx, mut rx) = tokio::sync::oneshot::channel::<Result<Value, ToolUseError>>();

        // release the GIL to allow the thread to run
        py.allow_threads(move || {