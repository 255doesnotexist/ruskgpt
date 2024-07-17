use reqwest::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};
use std::{error::Error, pin::Pin};
use futures::{Stream, StreamExt, TryStreamExt};
use serde_json::Value;
use log::{info, error};
use crate::process_response;

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
            crate::config::AdapterConfig::Zhipu(cfg) => ApiClient {
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

    pub async fn openai_stream_request(&self, prompt: &str) -> Result<impl Stream<Item = Result<String, reqwest::Error>>, Box<dyn Error>> {
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

        let response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request_body)
            .send()
            .await?;

        let req_status = response.status();
        if !req_status.is_success() {
            let response_text = response.text().await?;
            error!("Error response: {:?}", response_text);
            process_response::process_openai_error_response(response_text);
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

    pub async fn claude_stream_request(&self, prompt: &str) -> Result<Pin<Box<dyn Stream<Item = Result<String, reqwest::Error>>>>, Box<dyn Error>> {
        let url = format!("{}/messages", self.base_url);
        let request_body = serde_json::json!({
            "model": self.default_model,
            "max_tokens": self.max_tokens,
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        info!("Sending POST request to URL: {}", url);
        info!("Request body: {}", request_body);

        let response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.token)
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()
            .await?;

        let req_status = response.status();
        if !req_status.is_success() {
            let error_text = response.text().await?;
            error!("Error response: {:?}", error_text);
            return Err(format!("Received error response: {:?}", req_status).into());
        }

        let response_text = response.text().await?;
        let parsed_response: Value = serde_json::from_str(&response_text)?;

        if parsed_response["type"] == "error" {
            let error_message = parsed_response["error"]["message"].as_str().unwrap_or("Unknown error");
            return Err(format!("API error: {}", error_message).into());
        }

        let content = parsed_response["content"][0]["text"].as_str().unwrap_or("").to_string();

        Ok(Box::pin(futures::stream::once(async { Ok(content) })))
    }

    pub async fn zhipu_stream_request(&self, prompt: &str) -> Result<impl Stream<Item = Result<String, reqwest::Error>>, Box<dyn Error>> {
        let url = format!("https://open.bigmodel.cn/api/paas/v4/chat/completions");
        let request_body = serde_json::json!({
            "model": self.default_model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
            "stream": true
        });

        info!("Sending POST request to URL: {}", url);
        info!("Request body: {}", request_body);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", self.token))?);

        let response = self.client.post(&url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        let req_status = response.status();
        if !req_status.is_success() {
            let error_text = response.text().await?;
            error!("Error response: {:?}", error_text);
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
