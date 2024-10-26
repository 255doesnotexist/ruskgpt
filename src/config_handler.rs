use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use dirs_next::home_dir;
use crate::config::{AdapterConfig, ChatGLMConfig, ClaudeConfig, Config, DefaultConfig, FunctionCallingConfig, LoggingConfig, OpenAIConfig};

pub fn get_config_file_path(matches: &ArgMatches) -> PathBuf {
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
        adapter: AdapterConfig::OpenAI(OpenAIConfig {
            base_url: "https://api.openai.com".to_string(),
            default_model: "text-davinci-003".to_string(),
            token: "".to_string(),
            temperature: 0.7,
            top_p: Some(1.0),
            max_tokens: 100,
            function_calling_config: Some(FunctionCallingConfig {
                mode: "ANY".to_string(),
                allowed_function_names: None,
                function_declaration_names: vec!["example_function".to_string()],
            }),
        }),
        logging: LoggingConfig {
            level: "info".to_string(),
        },
    };

    default_config.save(home_config_path.to_str().unwrap()).expect("Failed to save default config");

    println!("Created a default config file at {}. Please update it with your settings.", home_config_path.to_str().unwrap());

    home_config_path
}

pub fn handle_config_edit(matches: &ArgMatches, config: &mut Config, config_file_path: &PathBuf) -> bool {
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

pub fn get_adapter_config(config: &Config) -> &AdapterConfig {
    &config.adapter
}

pub fn get_adapter_function_calling_config(adapter_config: &AdapterConfig) -> Option<&FunctionCallingConfig> {
    match adapter_config {
        AdapterConfig::OpenAI(config) => config.function_calling_config.as_ref(),
        AdapterConfig::Claude(config) => config.function_calling_config.as_ref(),
        AdapterConfig::Zhipu(config) => config.function_calling_config.as_ref(),
    }
}