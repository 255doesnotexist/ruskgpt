mod config;
mod api_client;
mod logger;
mod cli;
mod config_handler;
mod api;
mod process_response;

use config::Config;
use tokio::main;

#[main]
async fn main() {
    // Initialize logger
    logger::initialize_logger();

    // Parse command line arguments
    let matches = cli::parse_command_line_arguments();

    // Determine config file path
    let config_file_path = config_handler::get_config_file_path(&matches);

    // Load config
    let mut config = Config::from_file(config_file_path.to_str().unwrap())
        .expect("Failed to load config");

    // Handle configuration updates or editing
    if config_handler::handle_config_update_or_edit(&matches, &mut config, &config_file_path) {
        return;
    }

    // Get the question
    let prompt = matches.get_one::<String>("question")
        .expect("Usage: ruskgpt <your_question>");

    // Get adapter config
    let adapter_config = config_handler::get_adapter_config(&config);

    // Process response stream based on adapter config
    api::process_response_stream(adapter_config, prompt).await;
}
