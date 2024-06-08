use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default: DefaultConfig,
    pub openai_adapter: AdapterConfig,
    pub claude_adapter: AdapterConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub adapter: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum AdapterConfig {
    OpenAI(OpenAIConfig),
    Claude(ClaudeConfig),
    ChatGLM(ChatGLMConfig),
}

#[derive(Debug, Deserialize)]
pub struct OpenAIConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChatGLMConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
}


#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}