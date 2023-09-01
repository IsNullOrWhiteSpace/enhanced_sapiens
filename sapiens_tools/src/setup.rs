//! Sapiens CLI library
use sapiens::tools::toolbox::Toolbox;

use crate::conclude::ConcludeTool;
use crate::python::PythonTool;

/// Assemble the toolbox of tools.
///
/// - Uses features to enable/disable tools.
/// - Gets API keys from environment variables.
/// - Uses environment variables to configure tools: HUE_BRIDGE_IP, HUE_USERNAME
pub async fn toolbox_from_env() -> Toolbox {
    let mut toolbox = Toolbox::default();

    #[cfg(feature = "search")]
    {
        u