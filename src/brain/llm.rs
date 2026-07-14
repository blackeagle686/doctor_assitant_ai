use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub struct LlmClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LlmClient {
    pub fn new(config: &crate::core::config::Config) -> Result<Self> {
        // Default to OpenRouter if configured, otherwise fallback to OpenAI
        let (base_url, api_key, model) = if !config.openrouter_api_key.is_empty() {
            (
                "https://openrouter.ai/api/v1/chat/completions".to_string(),
                config.openrouter_api_key.clone(),
                config.openrouter_model.clone(),
            )
        } else if !config.openai_api_key.is_empty() {
            (
                "https://api.openai.com/v1/chat/completions".to_string(),
                config.openai_api_key.clone(),
                config.llm_model.clone(),
            )
        } else {
            return Err(anyhow::anyhow!("Must provide OPENROUTER_API_KEY or OPENAI_API_KEY in configuration or .env"));
        };

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        })
    }

    pub async fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
        };

        let res = self.client.post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to send LLM request")?;

        if !res.status().is_success() {
            let error_text = res.text().await?;
            return Err(anyhow::anyhow!("LLM API error: {}", error_text));
        }

        let mut response: OpenAiResponse = res.json().await.context("Failed to parse LLM response")?;
        
        if let Some(choice) = response.choices.pop() {
            Ok(choice.message.content)
        } else {
            Err(anyhow::anyhow!("No choices returned from LLM"))
        }
    }
}
