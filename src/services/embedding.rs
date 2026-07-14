
use anyhow::Result;
use fastembed::{
    EmbeddingModel,
    InitOptions,
    TextEmbedding,
};

pub struct EmbeddingService {
    model: TextEmbedding,
}

impl EmbeddingService {
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2)
        )?;

        Ok(Self { model })
    }

    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(
            vec![text.to_string()],
            None,
        )?;

        Ok(embeddings.into_iter().next().unwrap())
    }

    pub fn embed_batch(
        &self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>> {
        Ok(self.model.embed(texts, None)?)
    }
}