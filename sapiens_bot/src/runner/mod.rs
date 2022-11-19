use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

use sapiens::context::{ChatEntryFormatter, ContextDump, MessageFormatter};
use sapiens::models::SupportedModel;
use sapiens::tools::toolbox::Toolbox;
use sapiens::tools::TerminationMessage;
use sapiens::{
    