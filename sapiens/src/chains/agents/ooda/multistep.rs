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

- Action response will be provided. 
- Never produce the response of an Action. 
- Only use YAML for the Action.
- The loop will repeated until you have the answer to the original question. 
- No task is complete until the Conclude Tool is used to provide the answer. 
";

const TOOL_PREFIX: &str = r"
# The following are the ONLY Tools one can use for the Actions:
";

const OBSERVER_RESPONSE_FORMAT: &str = r"
# Format of your response

You must use the following format for your response. Comments are in bold and should be removed from your response.
====================
## Observations: 
**What do you know to be true? What do you you don't know? What are your sources? Note down important information for later.**
- <...>
====================
";

const ORIENTER_RESPONSE_FORMAT: &str = r"
# Format of your response

You must use the following format for your response. Comments are in bold and should be removed from your response.
====================
## Orientation: 
**Plan the intermediate objectives to answer complete the original task. 