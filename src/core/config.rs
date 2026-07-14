use std::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub max_tokens: usize,
    pub audio_max_size: usize,
    pub llm_model: String,
    pub openai_api_key: String,
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub qdrant_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let max_tokens = env::var("MAX_TOKENS")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .unwrap_or(1000);
            
        let audio_max_size = env::var("AUDIO_MAX_SIZE")
            .unwrap_or_else(|_| "10485760".to_string())
            .parse()
            .unwrap_or(10485760);

        let llm_model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4-turbo".to_string());
            
        let openai_api_key = env::var("OPENAI_API_KEY")
            .unwrap_or_else(|_| "".to_string());
            
        let openrouter_api_key = env::var("OPENROUTER_API_KEY")
            .unwrap_or_else(|_| "".to_string());
            
        let openrouter_model = env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "meta-llama/llama-3-70b-instruct".to_string());

        let qdrant_url = env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6334".to_string());
            
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

        Ok(Config {
            max_tokens,
            audio_max_size,
            llm_model,
            openai_api_key,
            openrouter_api_key,
            openrouter_model,
            qdrant_url,
            redis_url,
        })
    }
}