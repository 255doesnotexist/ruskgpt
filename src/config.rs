use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub default: DefaultConfig,
    pub adapter: AdapterConfig,
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
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClaudeConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGLMConfig {
    pub base_url: String,
    pub default_model: String,
    pub token: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionCallingConfig {
    pub mode: String,
    pub allowed_function_names: Option<Vec<String>>,
    pub function_declaration_names: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content: String = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
