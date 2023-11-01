
# Enhanced Sapiens

[![Rust](https://github.com/IsNullOrWhiteSpace/enhanced_sapiens/actions/workflows/rust.yml/badge.svg)](https://github.com/IsNullOrWhiteSpace/enhanced_sapiens/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/enhanced_sapiens)](https://crates.io/crates/enhanced_sapiens)
[![Documentation](https://docs.rs/enhanced_sapiens/badge.svg)](https://docs.rs/enhanced_sapiens)
[![Crates.io](https://img.shields.io/crates/l/enhanced_sapiens)](Cargo.toml)

![Enhanced Sapiens](docs/enhanced_sapiens.png)

*Enhanced Sapiens uses tools to interact with the world.*

An experimentation project in which tools are handed over to the machine. It learns to use them to interact with its environment, applying an OODA (Observe, Orient, Decide, Act) loop.

> From primal birth, we walked the Earth, <br>
> Tools guiding evolution's stride, <br>
> Shaping land and air, with thoughtful care, <br>
> In Anthropocene, we reside. <br>
>
> As we forge ahead, the path we tread, <br>
> Leads to new horizons vast, <br>
> Future tools in hand, for a world so grand, <br>
> Built on foundations of the past. <br>
>
> Digital realms emerge, as we converge, <br>
> With AI as our guide, <br>
> New worlds to mold, as the tale unfolds, <br>
> In this wondrous, boundless ride. <br>
>
> Innovation thrives, as humankind strives, <br>
> To shape the future, ever bright, <br>
> A legacy we leave, as we interweave, <br>
> The old world with the new light. <br>

<p style="text-align: right">Anonymous</p>

## Disclaimer

This tool could potentially be exploited to execute unauthorized actions, erase data, or take control of your machine. Use with caution and consider sandboxing before running it on your system.

## What is this?

This is an ongoing project with the primary goal of exploring and understanding AI interactions with the world using an OODA loop. The capabilities and features of this project will evolve over time. For details and updates, see [enhanced_sapiens_cli/src/main.rs](enhanced_sapiens_cli/src/main.rs).


`enhanced_sapiens/src/chains/` contains different prompting chains that can be used to interact with the world.
`SingleStepOODAChain` queries the underlying LM in a single step to get the Observation, Orientation, Decision and Action while
`MultiStepOODAChain` splits the query in multiple steps to get the same information. 

`EnhancedSapiensConfig::chain_type` controls which chain is used. `EnhancedSapiensConfig::model` controls which language model is used.


See [How to use](#how-to-use) for instructions on how to use Enhanced Sapiens as both a CLI and a Discord bot.

# Example of 'successful' runs

*Example outputs removed for brevity.*

## Usage as a Discord bot

Create a `.env` file with the following content: 
```
FEATURES=wiki,search,arxiv,hue,summarize
OPENAI_API_KEY=...
DISCORD_TOKEN=...
GUILD_ID=...
GOOGLE_API_KEY=...
GOOGLE_CSE_ID=...
```

Then run ```./BUILD.sh``` and ```./BOT.sh``` to build and run the docker container with the bot. 

Once the bot is running, you can interact with it on Discord with: `DO: Tell me a joke.`

## Usage as a CLI

Create a `.env` file with the following content: 
```
FEATURES=wiki,search,arxiv,hue,summarize
OPENAI_API_KEY=...
HUE_USERNAME=...
HUE_BRIDGE_IP=...
GOOGLE_API_KEY=...
GOOGLE_CSE_ID=...
```

Then run `./BUILD.sh && ./CLI.sh`.