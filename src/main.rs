mod config;
mod api_client;

use config::{Config, AdapterConfig};
use api_client::ApiClient;
use tokio::main;
use futures::StreamExt;
use std::env;
use log::{info, error};
use flexi_logger::{Logger, FileSpec, Criterion, Naming, Cleanup};
use chrono::Local;

#[main]
async fn main() {
    // Initialize logger
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

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: ruskgpt <your_questions>");
        return;
    }
    
    let config = Config::from_file("config.toml").expect("Failed to load config");

    let adapter_config = match config.default.adapter.as_str() {
        "openai_adapter" => config.openai_adapter,
        "claude_adapter" => config.claude_adapter,
        _ => panic!("Unsupported adapter specified in the configuration"),
    };

    let client = ApiClient::new(adapter_config);

    let prompt = &args[1..].join(" ");
    
    // Send request and process response stream
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
