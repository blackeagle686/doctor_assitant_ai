use anyhow::Result;
use crate::services::speech_recognition::{RecognizerFactory, RecognizerType};

pub async fn run_recognition(file_path: &str, use_local: bool) -> Result<String> {
    let rec_type = if use_local {
        RecognizerType::Local
    } else {
        RecognizerType::OpenAI
    };

    println!("Initializing speech recognizer...");
    let recognizer = RecognizerFactory::create(rec_type).await?;

    println!("Starting recognition pipeline step...");
    let transcript = recognizer.recognize(file_path).await?;
    
    Ok(transcript)
}
