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
        use crate::search::SearchTool;

        toolbox.add_tool(SearchTool::default()).await;
    }

    #[cfg(feature = "hue")]
    {
        use std::net::IpAddr;
        use std::str::FromStr;

        use huelib2::bridge;

        let bridge_ip = match std::env::var("HUE_BRIDGE_IP") {
            Ok(ip) => IpAddr::from_str(&ip).expect("Invalid IP address"),
            Err(_) => {
                println!("HUE_BRIDGE_IP env not set. Trying to discover bridge.");
                let bridge_ip = bridge::discover_nupnp().unwrap().pop().unwrap();
                println!(
                    "Discovered bridge at IP address: HUE_BRIDGE_IP={}",
                    bridge_ip
                );
                bridge_ip
            }
        };

        let username = match std::env::var("HUE_USERNAME") {
            Ok(username) => username,
            Err(_) => {
                println!("HUE_USERNAME env not set. Trying to register a new user.");

                // Register a new user.
                let username =
                    