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
use functional_calling::{list_function_declarations, load_function_declaration};
use tokio::main;

#[main]
async fn main() {
    // Initialize logger
    logger::initialize_logger();

    // Parse command line arguments
    let matches = cli::parse_command_line_arguments();

    // Check if list-functions flag is set
    if matches.get_flag("list-functions") {
        let functions = list_function_declarations().unwrap();
        for function in functions {
            println!("Function: {}", function.name);
            println!("Description: {}", function.description);
            for param in function.parameters {
            println!("  Param: {} ({}) - {}", param.name, param.param_type, param.description);
            }
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

    let list_functions = matches.get_flag("list-functions");
    if let Some(function_name) = matches.get_one::<String>("load-function") {
        match load_function_declaration(function_name) {
            Ok(function) => {
                println!("Function: {}", function.name);
                println!("Description: {}", function.description);
                for param in function.parameters {
                    println!("  Param: {} ({}) - {}", param.name, param.param_type, param.description);
                }
            }
            Err(e) => eprintln!("Error loading function: {}", e),
        }
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