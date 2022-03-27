use std::fmt::{Debug, Formatter};

use tracing::{debug, trace};

use crate::chains::agents::{format_outcome, Error};
use crate::chains::{Context, Message};
use crate::context::{ChatEntry, ChatHistory};
use crate::models::Role;
use crate::tools::toolbox::Toolbox;
use crate::{chains, prompt, SapiensConfig, WeakRuntimeObserver};

const PREFIX: &str = r"You are part of a group of cooperating assistants named Sapiens. Use available tools to answer the question as best as you can.
You will collectively proceed iteratively using an OODA loop. Don't overstep your role.

