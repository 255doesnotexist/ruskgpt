mod config;
mod api_client;

use config::{AdapterConfig, ClaudeConfig, Config, DefaultConfig, LoggingConfig, OpenAIConfig};
use api_client::ApiClient;
use tokio::main;
use futures::StreamExt;
use std::env;
use log::error;
use flexi_logger::{Logger, FileSpec, Criterion, Naming, Cleanup};
use clap::{Command, Arg};
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use dirs_next::home_dir;

#[main]
async fn main() {
    // Initialize logger
    initialize_logger();

    // Parse command line arguments
    let matches = parse_command_line_arguments();

    // Determine config file path
    let config_file_path = get_config_file_path(&matches);
    
    // Load config
    let mut config = Config::from_file(config_file_path.to_str().unwrap())
        .expect("Failed to load config");

    // Handle configuration updates or editing
    if handle_config_update_or_edit(&matches, &mut config, &config_file_path) {
        return;
    }

    // Get the question
    let prompt = matches.get_one::<String>("question")
        .expect("Usage: ruskgpt <your_question>");

    // Get adapter config and create client
    let adapter_config = get_adapter_config(&config);
    let client = ApiClient::new(adapter_config);

    // Send request and process response stream
    process_response_stream(client, prompt).await;
}

fn initialize_logger() {
    let temp_dir = std::env::temp_dir();
    let log_file_name = "ruskgpt";
    
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(temp_dir).basename(log_file_name))
        .rotate(
            Criterion::Size(10_000_000), // Rotate log file after it reaches 10 MB
            Naming::Timestamps,          // Use timestamps for rotated file names
            Cleanup::KeepLogFiles(7),    // Keep a maximum of 7 log files
        )
        .start()
        .unwrap();
}

fn parse_command_line_arguments() -> clap::ArgMatches {
    Command::new("ruskgpt")
        .version("
        0.0.1")
        .author("255doesnotexist <chenyejin2004@stumail.ysu.edu.cn>")
        .about("Yet another async AskGPT CLI client powered by Rust.")
        .arg(Arg::new("question")
            .help("The question to ask GPT")
            .index(1))
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .value_parser(clap::value_parser!(String))
            .help("Path to the configuration file"))
        .arg(Arg::new("set")
            .long("set")
            .value_parser(clap::value_parser!(String))
            .help("Set a configuration value in the format key=value"))
        .arg(Arg::new("edit")
            .short('e')
            .long("edit")
            .help("Open the configuration file in the default editor")
            .action(clap::ArgAction::SetTrue)) // 使用 SetTrue action
        .get_matches()
}

fn get_config_file_path(matches: &clap::ArgMatches) -> PathBuf {
    // Check if --config is specified and use it if present
    if let Some(config_path) = matches.get_one::<String>("config") {
        return PathBuf::from(config_path);
    }

    let home_config_dir = home_dir().unwrap().join(".ruskgpt");
    let home_config_path = home_config_dir.join("config.toml");

    // Check in home directory
    if home_config_path.exists() {
        return home_config_path;
    }

    // Check in current directory
    let current_config_path = PathBuf::from("config.toml");
    if current_config_path.exists() {
        // Attempt to load the config to ensure it's valid
        if Config::from_file(current_config_path.to_str().unwrap()).is_ok() {
            return current_config_path;
        } else {
            println!("Found an invalid config file in the current directory. Creating a default config.");
        }
    }

    // Create default config if not found or invalid
    if !home_config_dir.exists() {
        fs::create_dir_all(&home_config_dir).expect("Failed to create config directory");
    }

    let default_config = Config {
        default: DefaultConfig {
            adapter: "openai_adapter".to_string(),
        },
        openai_adapter: AdapterConfig::OpenAI(OpenAIConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            default_model: "text-davinci-003".to_string(),
            token: "your-openai-token".to_string(),
            temperature: 0.7,
            top_p: Some(1.0),
            max_tokens: 100,
        }),
        claude_adapter: AdapterConfig::Claude(ClaudeConfig {
            base_url: "https://api.anthropic.com/v1".to_string(),
            default_model: "claude-v1".to_string(),
            token: "your-claude-token".to_string(),
            temperature: 0.7,
            max_tokens: 100,
        }),
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    default_config.save(home_config_path.to_str().unwrap()).expect("Failed to save default config");

    println!("Created a default config file at {}. Please update it with your settings.", home_config_path.to_str().unwrap());

    home_config_path
}

fn handle_config_update_or_edit(matches: &clap::ArgMatches, config: &mut Config, config_file_path: &PathBuf) -> bool {
    if let Some(set_value) = matches.get_one::<String>("set") {
        let mut parts = set_value.splitn(2, '=');
        let key = parts.next().expect("Invalid format for --set");
        let value = parts.next().expect("Invalid format for --set");
        config.set_value(key, value);
        config.save(config_file_path.to_str().unwrap()).expect("Failed to save config");
        println!("Configuration updated successfully.");
        return true;
    }

    if matches.get_flag("edit") {
        open_config_file_in_editor(config_file_path);
        return true;
    }

    false
}

fn open_config_file_in_editor(config_file_path: &PathBuf) {
    let editors = ["code", "gedit", "nano", "vi", "notepad"];
    let editor = editors.iter()
        .find_map(|&editor| which::which(editor).ok())
        .unwrap_or_else(|| panic!("No suitable editor found"));

    ProcessCommand::new(editor)
        .arg(config_file_path.to_str().unwrap())
        .status()
        .expect("Failed to open editor");
}

fn get_adapter_config(config: &Config) -> AdapterConfig {
    match config.default.adapter.as_str() {
        "openai_adapter" => config.openai_adapter.clone(),
        "claude_adapter" => config.claude_adapter.clone(),
        _ => panic!("Unsupported adapter specified in the configuration"),
    }
}

async fn process_response_stream(client: ApiClient, prompt: &str) {
    match client.stream_request(prompt).await {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        print!("{}", response);
                    },
                    Err(e) => {
                        error!("Error: {}", e);
                    },
                }
            }
        }
        Err(e) => {
            error!("Failed to send request: {}", e);
        }
    }
}
