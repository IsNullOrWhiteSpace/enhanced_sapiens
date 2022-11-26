use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

use sapiens::context::{ChatEntryFormatter, ContextDump, MessageFormatter};
use sapiens::models::SupportedModel;
use sapiens::tools::toolbox::Toolbox;
use sapiens::tools::TerminationMessage;
use sapiens::{
    models, wrap_observer, Error, InvalidInvocationNotification, InvocationFailureNotification,
    InvocationResultNotification, InvocationSuccessNotification, MessageNotification,
    ModelNotification, RuntimeObserver, SapiensConfig, TaskState, WeakRuntimeObserver,
};
use serenity::futures::channel::mpsc;
use serenity::futures::{SinkExt, StreamExt};
use tracing::{debug, error, info, warn};

use crate::runner::utils::{sanitize_msgs_for_discord, Formatter};

/// Formatting utilities
pub mod utils;

/// Sapiens bot
pub struct SapiensBot {
    toolbox: Toolbox,
    config: SapiensConfig,
}

impl SapiensBot {
    /// Create a new bot from the environment variables: OPENAI_API_KEY, ...
    pub async fn new_from_env() -> Self {
        let toolbox = sapiens_tools::setup::toolbox_from_env().await;

        let _ =
            std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in configuration file");

        let temperature = Some(0.);

        let model = match std::env::var("MODEL") {
            Ok(e) => SupportedModel::from_str(&e).expect("Invalid model"),
            Err(e) => {
                if e == VarError::NotPresent {
                    warn!("MODEL not specified: defaulting to chat-bison-001.");
                    SupportedModel::ChatBison001
                } else {
                    panic!("Invalid model: {}", e)
                }
            }
        };

        let model = match model {
            SupportedModel::ChatBison001 => {
                let google_api_key =
                    std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY is not set");

                models::vertex_ai::build(google_api_key, temperature)
                    .await
                    .expect("Failed to build model")
            }
            _ => {
                let api_key = std::env::var("OPENAI_API_KEY").ok();
                let api_base = std::env::var("OPENAI_API_BASE").ok();

                models::openai::build(model, api_key, api_base, temperature)
                    .await
                    .expect("Failed to build model")
            