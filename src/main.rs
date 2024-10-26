mod config;
mod api_client;
mod logger;
mod cli;
mod config_handler;
mod api;
mod process_response;
mod functional_calling;

use api_client::ApiClient;
use config::Config;
use functional_calling::{list_function_declarations, load_function_declaration, FunctionDeclaration};
use tokio::main;

#[main]
async fn main() {
    // Initialize logger
    logger::initialize_logger();

    // Parse command line arguments
    let matches = cli::parse_command_line_arguments();

    // Check if list-functions flag is set
    let list_functions = matches.get_flag("list-functions");

    if list_functions {
        match list_function_declarations() {
            Ok(functions) => {
                for function in functions {
                    match function {
                        FunctionDeclaration::Shell { name, description, parameters, command_template } => {
                            println!("Function: {}", name);
                            println!("Description: {}", description);
                            println!("Command Template: {}", command_template);
                            for param in parameters {
                                println!("  Param: {} ({}) - {} [Dangerous: {}]", param.name, param.param_type, param.description, param.dangerous.unwrap_or(false));
                            }
                        }
                        FunctionDeclaration::Interactive { name, description, parameters } => {
                            println!("Function: {}", name);
                            println!("Description: {}", description);
                            for param in parameters {
                                println!("  Param: {} ({}) - {} [Dangerous: {}]", param.name, param.param_type, param.description, param.dangerous.unwrap_or(false));
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error listing functions: {}", e),
        }
        return;
    }

    // Determine config file path
    let config_file_path = config_handler::get_config_file_path(&matches);

    // Load config
    let mut config = Config::from_file(config_file_path.to_str().unwrap())
        .expect("Failed to load config");

    // Handle configuration updates or editing
    if config_handler::handle_config_edit(&matches, &mut config, &config_file_path) {
        return;
    }

    // Get the question
    let prompt = matches.get_one::<String>("question")
        .expect("Usage: ruskgpt <your_question>");

    // Get adapter config
    let adapter_config = config_handler::get_adapter_config(&config);

    // Check if agent functionality is enabled
    let enable_agent = matches.get_flag("agent");

    // Create API client
    let client = ApiClient::new(adapter_config.clone());

    if enable_agent {
        // Placeholder for future workflow logic
        panic!("Agent functionality is not yet implemented");
    } else {
        // Process response stream
        api::process_response_stream(adapter_config, prompt).await;
    }
}