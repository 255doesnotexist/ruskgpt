use crate::api_client::ApiClient;
use futures::StreamExt;
use log::error;

pub async fn process_response_stream(client: ApiClient, prompt: &str) {
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
