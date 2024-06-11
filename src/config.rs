use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub default: DefaultConfig,
    pub openai_adapter: AdapterConfig,
    pub claude_adapter: AdapterConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultConfig {
    pub adapter: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum AdapterConfig {
    OpenAI(OpenAIConfig),
    Claude(ClaudeConfig),
    Zhipu(ChatGLMConfig),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClaudeConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGLMConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        match key {
            "default.adapter" => self.default.adapter = value.to_string(),
            "openai_adapter.base_url" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.base_url = value.to_string();
                }
            },
            "openai_adapter.default_model" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.default_model = value.to_string();
                }
            },
            "openai_adapter.token" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.token = value.to_string();
                }
            },
            "openai_adapter.temperature" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.temperature = value.parse().expect("Invalid value for temperature");
                }
            },
            "openai_adapter.top_p" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.top_p = Some(value.parse().expect("Invalid value for top_p"));
                }
            },
            "openai_adapter.max_tokens" => {
                if let AdapterConfig::OpenAI(ref mut cfg) = self.openai_adapter {
                    cfg.max_tokens = value.parse().expect("Invalid value for max_tokens");
                }
            },
            "claude_adapter.base_url" => {
                if let AdapterConfig::Claude(ref mut cfg) = self.claude_adapter {
                    cfg.base_url = value.to_string();
                }
            },
            "claude_adapter.default_model" => {
                if let AdapterConfig::Claude(ref mut cfg) = self.claude_adapter {
                    cfg.default_model = value.to_string();
                }
            },
            "claude_adapter.token" => {
                if let AdapterConfig::Claude(ref mut cfg) = self.claude_adapter {
                    cfg.token = value.to_string();
                }
            },
            "claude_adapter.temperature" => {
                if let AdapterConfig::Claude(ref mut cfg) = self.claude_adapter {
                    cfg.temperature = value.parse().expect("Invalid value for temperature");
                }
            },
            "claude_adapter.max_tokens" => {
                if let AdapterConfig::Claude(ref mut cfg) = self.claude_adapter {
                    cfg.max_tokens = value.parse().expect("Invalid value for max_tokens");
                }
            },
            "logging.level" => self.logging.level = value.to_string(),
            _ => panic!("Unknown configuration key: {}", key),
        }
    }

    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let config_content = toml::to_string(self)?;
        fs::write(file_path, config_content)?;
        Ok(())
    }
}
