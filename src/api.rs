use crate::api_client::ApiClient;
use crate::config::AdapterConfig;
use futures::StreamExt;
use log::error;

pub async fn process_response_stream(adapter_config: AdapterConfig, prompt: &str) {
    match adapter_config {
        AdapterConfig::OpenAI(config) => {
            process_openai_request(ApiClient::new(AdapterConfig::OpenAI(config)), prompt).await;
        },
        AdapterConfig::Claude(config) => {
            process_claude_request(ApiClient::new(AdapterConfig::Claude(config)), prompt).await;
        },
        AdapterConfig::Zhipu(config) => {
            process_chatglm_request(ApiClient::new(AdapterConfig::Zhipu(config)), prompt).await;
        },
    }
}

async fn process_openai_request(client: ApiClient, prompt: &str) {
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

async fn process_claude_request(client: ApiClient, prompt: &str) {
    // TODO: Implement Claude specific request processing
    todo!("Claude adapter request processing is not implemented yet. Sooner or later. Fucked by life so not sure.");
}

async fn process_chatglm_request(client: ApiClient, prompt: &str) {
    // TODO: Implement ChatGLM specific request processing
    todo!("ChatGLM adapter request processing is not implemented yet.");
}