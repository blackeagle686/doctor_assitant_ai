use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::services::speech_recognition::SpeechRecognizer;

pub struct LocalRecognizer {
    model_path: String,
}

impl LocalRecognizer {
    pub async fn new() -> Result<Self> {
        println!("Initializing Local Recognizer...");
        let model_path = "ggml-base.bin".to_string();
        
        if !Path::new(&model_path).exists() {
            println!("Downloading Whisper base model (this might take a few minutes)...");
            // ggml-base.bin works for multiple languages including Arabic
            let url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin";
            let response = reqwest::get(url).await?.bytes().await?;
            let mut file = File::create(&model_path).await?;
            file.write_all(&response).await?;
            println!("Model downloaded successfully.");
        } else {
            println!("Local Whisper model found.");
        }

        Ok(Self { model_path })
    }
}

#[async_trait]
impl SpeechRecognizer for LocalRecognizer {
    async fn recognize(&self, file_path: &str) -> Result<String> {
        println!("Reading audio file for Local Whisper: {}", file_path);
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("File not found: {}", file_path));
        }

        let file_path_str = file_path.to_string();
        let model_path_str = self.model_path.clone();

        // Run the blocking Whisper operations inside a spawn_blocking task
        let full_text = tokio::task::spawn_blocking(move || -> Result<String> {
            let mut reader = hound::WavReader::open(Path::new(&file_path_str))
                .context("Failed to open wav file")?;
            
            let spec = reader.spec();
            
            // Convert samples to f32 as required by Whisper
            let samples: Vec<f32> = if spec.sample_format == hound::SampleFormat::Float {
                reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect()
            } else {
                // Assuming 16-bit PCM if not float
                reader.samples::<i16>().map(|s| {
                    let val = s.unwrap_or(0);
                    val as f32 / 32768.0
                }).collect()
            };

            let ctx_params = WhisperContextParameters::default();
            let context = WhisperContext::new_with_params(&model_path_str, ctx_params)
                .context("Failed to load Whisper context")?;
            
            let mut state = context.create_state().context("Failed to create Whisper state")?;
            
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            // Set language to Arabic to help with mixed Arabic/English (Egyptian Arabic)
            params.set_language(Some("ar"));
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            println!("Running Local Whisper inference...");
            state.full(params, &samples).context("Failed to run whisper inference")?;

            let num_segments = state.full_n_segments();
            let mut text = String::new();
            for i in 0..num_segments {
                let segment = state.get_segment(i).context("Failed to get segment text")?;
                text.push_str(&segment.to_str().unwrap());
            }

            Ok(text)
        }).await??;

        Ok(full_text)
    }
}
