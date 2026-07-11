use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::multipart;
use serde::Deserialize;
use std::env;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::services::speech_recognition::SpeechRecognizer;

#[derive(Deserialize)]
struct WhisperResponse {
    text: String,
}

pub struct OpenAiRecognizer {
    api_key: String,
}

impl OpenAiRecognizer {
    pub fn new() -> Result<Self> {
        println!("Loading OpenAI configuration...");
        dotenv::dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not found. Please set it to use the recognition service.")?;
        Ok(Self { api_key })
    }
}

#[async_trait]
impl SpeechRecognizer for OpenAiRecognizer {
    async fn recognize(&self, file_path: &str) -> Result<String> {
        println!("Reading audio file for OpenAI: {}", file_path);
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Audio file not found: {}", file_path));
        }

        let mut file = File::open(path).await.context("Failed to open audio file")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.context("Failed to read audio file into memory")?;

        let file_part = multipart::Part::bytes(buffer)
            .file_name(path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("audio/wav")?;

        let prompt = "هذا تسجيل لمريض مصري، يتحدث باللهجة المصرية، وقد يذكر بعض المصطلحات الطبية أو الكلمات الإنجليزية مثل blood pressure أو scan.";

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("model", "whisper-1")
            .text("prompt", prompt)
            .text("language", "ar");

        println!("Sending audio to OpenAI Whisper API for speech-to-text recognition...");
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;

        if !res.status().is_success() {
            let error_text = res.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let whisper_res: WhisperResponse = res.json().await.context("Failed to parse OpenAI API response as JSON")?;

        Ok(whisper_res.text)
    }
}
