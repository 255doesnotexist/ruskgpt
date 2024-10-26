use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
    pub dangerous: Option<bool>, // 是否需要执行前检查
    pub flag: Option<String>, // 用于布尔类型参数的命令行标志
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_command() {
        // 从 abilities/ls.toml 中加载功能声明
        let function = load_function_declaration("ls").expect("Failed to load function declaration");

        // 测试不带参数的命令生成
        let llm_params = [];
        let command = generate_command(function.clone(), &llm_params);
        assert_eq!(command, "ls");

        // 测试带有 -a 参数的命令生成
        let llm_params_with_hidden = [("all", "true")];
        let command_with_hidden = generate_command(function.clone(), &llm_params_with_hidden);
        assert_eq!(command_with_hidden, "ls -a");

        // 测试带有 -l 参数的命令生成
        let llm_params_with_long = [("long", "true")];
        let command_with_long = generate_command(function.clone(), &llm_params_with_long);
        assert_eq!(command_with_long, "ls -l");

        // 测试带有 -a 和 -l 参数的命令生成
        let llm_params_with_all_and_long = [("all", "true"), ("long", "true")];
        let command_with_all_and_long = generate_command(function, &llm_params_with_all_and_long);
        assert_eq!(command_with_all_and_long, "ls -a -l");

        // 从 abilities/eval.toml 中加载功能声明
        let function_eval = load_function_declaration("eval").expect("Failed to load function declaration");

        // 测试 eval 命令生成
        let llm_params_eval = [("command", "echo Hello, world!")];
        let command_eval = generate_command(function_eval, &llm_params_eval);
        assert_eq!(command_eval, "echo Hello, world!");
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum FunctionDeclaration {
    Shell {
        name: String,
        description: String,
        parameters: Vec<FunctionParameter>,
        command_template: String, // 用于指导程序如何组织 shell 命令
    },
    Interactive {
        name: String,
        description: String,
        parameters: Vec<FunctionParameter>,
        prompt: String, // 提示用户输入
        regex: String, // 验证用户输入的正则表达式
    },
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

pub fn generate_command(function: FunctionDeclaration, llm_params: &[(&str, &str)]) -> String {
    let mut command = match &function {
        FunctionDeclaration::Shell { command_template, .. } => command_template.clone(),
        _ => {
            panic!("This function type won't generate a command");
        },
    };

    let parameters: HashMap<_, _> = match &function {
        FunctionDeclaration::Shell { parameters, .. } => parameters.iter().map(|p| (p.name.clone(), p.clone())).collect(),
        _ => {
            panic!("This function type won't generate a command");
        },
    };

    let llm_params_map: HashMap<_, _> = llm_params.iter().cloned().collect();

    for (key, param) in &parameters {
        let placeholder = format!("{{{}}}", key);
        if let Some(value) = llm_params_map.get(key.as_str()) {
            if *value == "true" {
                // 如果参数值为 true，则替换为参数的 flag
                if let Some(flag) = &param.flag {
                    command = command.replace(&placeholder, flag);
                }
            } else if *value == "false" {
                // 如果参数值为 false，则移除占位符，以及它前面那个空格
                command = command.replace(&format!(" {}", &placeholder), "");
            } else {
                // 否则，替换为实际的参数值
                command = command.replace(&placeholder, value);
            }
        } else {
            // 如果参数未提供，则移除占位符，以及它前面那个空格
            command = command.replace(&format!(" {}", &placeholder), "");
        }
    }
    command.trim().to_owned()
}