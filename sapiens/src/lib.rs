//! Sapiens library
//!
//! *Sapiens uses tools to interact with the world.*
//!
//! An experiment with handing over the tools to the machine.
//!
//! # Overview
//! This library is the core of Sapiens. It contains the logic for the
//! interaction between the user, the language model and the tools.
//!
//! # More information
//! See https://github.com/ssoudan/sapiens/tree/main/sapiens_cli for an example of usage or
//! https://github.com/ssoudan/sapiens/tree/main/sapiens_bot for a Discord bot.
//!
//! https://github.com/ssoudan/sapiens/tree/main/sapiens_exp is a framework to run experiments and collect traces
//! of the interactions between the language model and the tools to accomplish a
//! task.
//!
//! A collection of tools is defined in https://github.com/ssoudan/sapiens/tree/main/sapiens_tools.
pub mod context;

/// Prompt generation logic
pub mod prompt;

/// Toolbox for sapiens
pub mod tools;

/// Language models
pub mod models;

pub mod chains;

use std::fmt::Debug;
use std::str::FromStr;
use std::sync::{Arc, Weak};

use clap::builder::PossibleValue;
use serde::{Deserialize, Serialize};
use tokio::sync::Mu