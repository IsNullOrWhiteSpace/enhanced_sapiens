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
**Plan the intermediate objectives to answer complete the original task. Maintain a list of current objectives updated as you go.**
- <...>
```
====================
";

const DECIDER_RESPONSE_FORMAT: &str = r"
# Format of your response

You must use the following format for your response. Comments are in bold and should be removed from your response.
====================
## Decision: 
**Decide what to do first to answer the question. Why? How will you if it succeeds? How will you if it fails?**
- <...>
====================
";

const ACTOR_RESPONSE_FORMAT: &str = r"
# Format of your response

You must use the following format for your response. Comments are in bold and should be removed from your response.
====================
## The ONLY Action: 
**Take a single Action consisting of exactly one pair of `tool_name` and `parameters`. Never give more than one YAML. **
```yaml
tool_name: <ToolName>
parameters:
    <...>  
```
We will take further action based on the response.
====================

Notes: 
- Action has the following fields: `tool_name` and `parameters` ONLY.
- `parameters` uses the format specified for the Tool.
- One Action at a time. No more. No less.
";

const OBSERVER_PROTO_INITIAL_RESPONSE: &str = r#"
## Observations:
- The given list to sort is [2, 3, 1, 4, 5].
- I need to sort this list in ascending order.
"#;

const OBSERVER_PROTO_SECOND_INPUT: &str = r#"
## Orientation:
- SandboxedPython can be used to sort the list.
- I need to provide only the `tool_name` and `parameters` fields for the SandboxedPython Tool.
- I expect the response of the Action to contains the field `stdout` with the sorted list and `stderr` empty.
- I need to use the Conclude Tool to terminate the task when I have the sorted list in plain text.
## Decision:
- We can use the sorted() function of Python to sort the list.
## The ONLY Action:
```yaml
tool_name: SandboxedPython
parameters:
  code: |
    lst = [2, 3, 1, 4, 5]
    sorted_list = sorted(lst)
    print(f"The sorted list is {sorted_list}")
```
We will take further action based on the response.
# Action SandboxedPython response:
```yaml
stdout: |
  The sorted list is [1, 2, 3, 4, 5]
stderr: ''
```
"#;

const OBSERVER_PROTO_SECOND_RESPONSE: &str = r"
## Observations:
- We needed to sort the list in ascending order.
- We have the response of the Action.
- We have the sorted list: [1, 2, 3, 4, 5].
";

const ORIENTER_PROTO_INITIAL_RESPONSE: &str = r#"
## Orientation:
- SandboxedPython can be used to sort the list.
- I need to provide only the `tool_name` and `parameters` fields for the SandboxedPython Tool.
- I expect the response of the Action to contains the field `stdout` with the sorted list and `stderr` empty.
- I need to use the Conclude Tool to terminate the task when I have the sorted list in plain text.
"#;

const ORIENTER_PROTO_SECOND_INPUT: &str = r#"
## Decision:
- We can use the sorted() function of Python to sort the list.
## The ONLY Action:
```yaml
tool_name: SandboxedPython
parameters:
  code: |
    lst = [2, 3, 1, 4, 5]
    sorted_list = sorted(lst)
    print(f"The sorted list is {sorted_list}")
```
We will take further action based on the response.
# Action SandboxedPython response:
```yaml
stdout: |
  The sorted list is [1, 2, 3, 4, 5]
stderr: ''
```
## Observations:
- We needed to sort the list in ascending order.
- We have the response of the Action.
- We have the sorted list: [1, 2, 3, 4, 5].
"#;

const ORIENTER_PROTO_SECOND_RESPONSE: &str = r"
## Orientation:
- I know the answer to the original question.
- I need to provide the `tool_name` and `parameters` fields for the Conclude Tool.
";

const DECIDER_PROTO_INITIAL_RESPONSE: &str = r#"
## Decision:
- We can use the sorted() function of Python to sort the list.
"#;

const DECIDER_PROTO_SECOND_INPUT: &str = r#"
## The ONLY Action:
```yaml
tool_name: SandboxedPython
parameters:
  code: |
    lst = [2, 3, 1, 4, 5]
    sorted_list = sorted(lst)
    print(f"The sorted list is {sorted_list}")
```
We will take further action based on the response.
# Action SandboxedPython response:
```yaml
stdout: |
  The sorted list is [1, 2, 3, 4, 5]
stderr: ''
```
## Observations:
- We needed to sort the list in ascending order.
- We have the response of the Action.
- We have the sorted list: [1, 2, 3, 4, 5].
## Orientation:
- I know the answer to the original question.
- I need to provide the `tool_name` and `parameters` fields for the Conclude Tool.
"#;

const DECIDER_PROTO_SECOND_RESPONSE: &str = r"
## Decision:
- Use the Conclude Tool to terminate the task with the sorted list.
";

const ACTOR_PROTO_INITIAL_RESPONSE: &str = r#"
## The ONLY Action:
```yaml
tool_name: SandboxedPython
parameters:
  code: |
    lst = [2, 3, 1, 4, 5]
    sorted_list = sorted(lst)
    print(f"The sorted list is {sorted_list}")
```
That's it for now. We will take further action based on the response.
"#;

const ACTOR_PROTO_SECOND_INPUT: &str = r#"
# Action SandboxedPython response:
```yaml
stdout: |
  The sorted list is [1, 2, 3, 4, 5]
stderr: ''
```
## Observations:
- We needed to sort the list in ascending order.
- We have the response of the Action.
- We have the sorted list: [1, 2, 3, 4, 5].
## Orientation:
- I know the answer to the original question.
- I need to provide the `tool_name` and `parameters` fields for the Conclude Tool.
## Decision:
- Use the Conclude Tool to terminate the task with the sorted list.
"#;

const ACTOR_PROTO_SECOND_RESPONSE: &str = r"
## The ONLY Action:
```yaml
tool_name: Conclude
parameters:
  original_question: |
    Sort in ascending order: [2, 3, 1, 4, 5]
  conclusion: |
    The ascending sorted list is [1, 2, 3, 4, 5].
```
";

enum AgentRole {
    Observer { prompt_manager: prompt::Manager },
    Orienter { prompt_manager: prompt::Manager },
    Decider { prompt_manager: prompt::Manager },
    Actor { prompt_manager: prompt::Manager },
}

impl Debug for AgentRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRole::Observer { .. } => write!(f, "Observer"),
            AgentRole::Orienter { .. } => write!(f, "Orienter"),
            AgentRole::Decider { .. } => write!(f, "Decider"),
            AgentRole::Actor { .. } => write!(f, "Actor"),
        }
    }
}

impl AgentRole {
    async fn convert_context_to_chat_history(
        &self,
        mut chat_history: ChatHistory,
        context: &Context,
    ) -> Result<ChatHistory, Error> {
        // build the examples
        let examples = self.build_examples();

        let prompt_manager = match self {
            AgentRole::Observer { prompt_manager } => prompt_manager,
            AgentRole::Orienter { prompt_manager } => prompt_manager,
            AgentRole::Decider { prompt_manager } => prompt_manager,
            AgentRole::Actor { prompt_manager } => prompt_manager,
        };

        // Add the prompts to the chat history
        prompt_manager
            .populate_chat_history(&mut chat_history, examples)
            .await;

        // Convert the context to a chat history
        // - get the latest 'Task' from the context
        let task = context.get_latest_task().unwrap();

        let task = prompt_manager.build_task_prompt(&task);

        // build the chat history from the context:
        // - group together Orientation, Decision, Action, ActionResult messages as a
        //   single chat entry from the User
        // - Observation m