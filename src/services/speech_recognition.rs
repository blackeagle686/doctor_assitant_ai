use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait SpeechRecognizer: Send + Sync {
    async fn recognize(&self, file_path: &str) -> Result<String>;
}

pub enum RecognizerType {
    OpenAI,
    Local,
}

pub struct RecognizerFactory;

impl RecognizerFactory {
    pub async fn create(rec_type: RecognizerType) -> Result<Box<dyn SpeechRecognizer>> {
        match rec_type {
            RecognizerType::OpenAI => {
                let recognizer = crate::services::openai_audio::OpenAiRecognizer::new()?;
                Ok(Box::new(recognizer))
            }
            RecognizerType::Local => {
                let recognizer = crate::services::local_audio::LocalRecognizer::new().await?;
                Ok(Box::new(recognizer))
            }
        }
    }
}
