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
    match client.openai_stream_request(prompt).await {
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
            error!("Error processing OpenAI request: {}", e);
        }
    }
}

async fn process_claude_request(client: ApiClient, prompt: &str) {
    match client.claude_stream_request(prompt).await {
        Ok(mut stream) => {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(content) => {
                        println!("{}", content);
                    }
                    Err(err) => {
                        error!("Error receiving stream chunk: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            error!("Error processing Claude request: {}", err);
        }
    }
}

async fn process_chatglm_request(client: ApiClient, prompt: &str) {
    // TODO: Implement ChatGLM specific request processing
    todo!("ChatGLM adapter request processing is not implemented yet.");
}