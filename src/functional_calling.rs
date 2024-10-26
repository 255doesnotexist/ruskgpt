use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: Vec<FunctionParameter>,
}

pub fn load_function_declaration(name: &str) -> Result<FunctionDeclaration, Box<dyn Error>> {
    let path = Path::new("abilities").join(format!("{}.toml", name));
    let content = fs::read_to_string(path)?;
    let function_declaration: FunctionDeclaration = toml::from_str(&content)?;
    Ok(function_declaration)
}

pub fn list_function_declarations() -> Result<Vec<FunctionDeclaration>, Box<dyn Error>> {
    let mut functions = Vec::new();
    for entry in fs::read_dir("abilities")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let content = fs::read_to_string(&path)?;
            let function_declaration: FunctionDeclaration = toml::from_str(&content)?;
            functions.push(function_declaration);
        }
    }
    Ok(functions)
}