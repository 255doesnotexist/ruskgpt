# Ruskgpt

Yet another async AskGPT CLI client powered by Rust.

## Overview

`ruskgpt` is a command-line interface (CLI) client for interacting with GPT models asynchronously. It is built with Rust to provide high performance and reliability.

## Features

- Asynchronous interactions with GPT models
- Configuration management
- Easy-to-use command-line interface
- Shell workflows based on function calling (Still working in progress)

## Installation

To install `ruskgpt`, ensure you have Rust and Cargo installed. Then, run the following command:

```sh
cargo install ruskgpt
```

Then put your OpenAI or other access token in configuration.

```
ruskgpt -e
```

## Usage
### Asking a Question
To ask a question, simply run:

```sh
ruskgpt "Why did the scarecrow win an award?"
# Because he was outstanding in his field!
```

### Configuration

To open the configuration file in the default editor, use the -e or --edit option:

```sh
ruskgpt -e
```

You can specify a configuration file with the --config option:

```sh
ruskgpt --config path/to/config.toml
```

# Supported LLM APIs

| API Provider | Supported | Notes |
|--------------|-----------| ----- |
| OpenAI        |    ✔️    | v1/chat/completions needed |
| OpenAI Like   |    ✔️    | v1/chat/completions needed |
| Claude        |    half    | Experimental (new message API) |
| ChatGLM       |    ❌    | TODO |
| Qwen          |    ❌    | TODO |
| Gemini        |    ❌    | TODO |
| Deepseek      |    ❌    | TODO |

## License
This project is licensed under the GPL-2.0 License - see the LICENSE file for details.

## Authors

<a href="https://github.com/255doesnotexist/ruskgpt/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=255doesnotexist/ruskgpt" />
</a>

## Inspirations
Inspired from [praeclarum/AskGPT](https://github.com/praeclarum/AskGPT), which [Jiang Yanyan](https://jyywiki.cn/) frequently used in his OS lectures.

## Supported Platforms
- Windows, Linux, macOS (x86, amd64, arm64)

## Stars

[![Star History Chart](https://api.star-history.com/svg?repos=255doesnotexist/ruskgpt&type=Date)](https://star-history.com/#255doesnotexist/ruskgpt)