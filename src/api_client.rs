use reqwest::Client;
use std::error::Error;
use futures::{Stream, StreamExt, TryStreamExt};
use serde_json::Value;
use log::{info, error};

pub struct ApiClient {
    client: Client,
    base_url: String,
    default_model: String,
    token: String,
    temperature: f32,
    top_p: Option<f32>,
    max_tokens: u32,
}

impl ApiClient {
    pub fn new(config: crate::config::AdapterConfig) -> Self {
        match config {
            crate::config::AdapterConfig::OpenAI(cfg) => ApiClient {
                client: Client::new(),
                base_url: cfg.base_url,
                default_model: cfg.default_model,
                token: cfg.token,
                temperature: cfg.temperature,
                top_p: cfg.top_p,
                max_tokens: cfg.max_tokens,
            },
            crate::config::AdapterConfig::Claude(cfg) => ApiClient {
                client: Client::new(),
                base_url: cfg.base_url,
                default_model: cfg.default_model,
                token: cfg.token,
                temperature: cfg.temperature,
                top_p: None, // Claude doesn't have top_p
                max_tokens: cfg.max_tokens,
            },
            crate::config::AdapterConfig::ChatGLM(cfg) => ApiClient {
                client: Client::new(),
                base_url: cfg.base_url,
                default_model: cfg.default_model,
                token: cfg.token,
                temperature: cfg.temperature,
                top_p: None, // Assuming ChatGLM doesn't have top_p
                max_tokens: cfg.max_tokens,
            },
        }
    }

    pub async fn stream_request(&self, prompt: &str) -> Result<impl Stream<Item = Result<String, reqwest::Error>>, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);
        let request_body = serde_json::json!({
            "model": self.default_model,
            "messages": [
                { "role": "system", "content": "You are a helpful assistant." },
                { "role": "user", "content": prompt }
            ],
            "temperature": self.temperature,
            "top_p": self.top_p.unwrap_or(1.0),
            "max_tokens": self.max_tokens,
            "stream": true,
        });

        info!("Sending POST request to URL: {}", url);
        info!("Request body: {}", request_body);

        let mut response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request_body)
            .send()
            .await?;

        let req_status = response.status();
        if !req_status.is_success() {
            error!("Error response: {:?}", response.text().await?);
            return Err(format!("Received error response: {:?}", req_status).into());
        }

        let byte_stream = response.bytes_stream();

        Ok(byte_stream.map_ok(|bytes| {
            let data = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| "".to_string());
            data.split("\n")
                .filter_map(|line| {
                    if line.starts_with("data:") {
                        Some(line[5..].trim().to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        }).try_filter(|data| futures::future::ready(!data.contains("[DONE]") && !data.trim().is_empty()))
        .flat_map(|result| {
            futures::stream::iter(match result {
                Ok(data) => data.split("}\n{").map(|chunk| {
                    if chunk.starts_with("{") && chunk.ends_with("}") {
                        Ok(chunk.to_string())
                    } else if chunk.starts_with("{") {
                        Ok(format!("{}{}", chunk, "}"))
                    } else if chunk.ends_with("}") {
                        Ok(format!("{}{}", "{", chunk))
                    } else {
                        Ok(format!("{}{}{}", "{", chunk, "}"))
                    }
                }).collect::<Vec<_>>(),
                Err(e) => vec![Err(e)],
            })
        })
        .map_ok(|chunk| {
            if let Ok(parsed) = serde_json::from_str::<Value>(&chunk) {
                if let Some(content) = parsed["choices"][0]["delta"]["content"].as_str() {
                    content.to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        })
        .filter(|content| futures::future::ready(content.is_ok() && !content.as_ref().unwrap().is_empty())))
    }
}
