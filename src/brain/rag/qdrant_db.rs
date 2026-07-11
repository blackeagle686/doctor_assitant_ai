use anyhow::{Context, Result};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{PointStruct, SearchPoints};
use std::env;

pub struct QdrantDb {
    client: QdrantClient,
    collection_name: String,
}

impl QdrantDb {
    pub async fn new() -> Result<Self> {
        let uri = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());
        let collection_name = "medical_knowledge".to_string();
        
        println!("Connecting to Qdrant at {}", uri);
        let client = QdrantClient::from_url(&uri).build()?;

        Ok(Self { client, collection_name })
    }

    pub async fn search(&self, query_vector: Vec<f32>, limit: u64) -> Result<Vec<String>> {
        let search_request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_vector,
            limit,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let response = self.client.search_points(&search_request).await.context("Failed to search Qdrant")?;
        
        let mut results = Vec::new();
        for point in response.result {
            if let Some(payload) = point.payload.get("text") {
                if let Some(text) = payload.as_str() {
                    results.push(text.to_string());
                }
            }
        }

        Ok(results)
    }

    pub async fn insert(&self, id: u64, vector: Vec<f32>, text: &str) -> Result<()> {
        let payload = serde_json::json!({"text": text}).try_into()?;
        
        let point = PointStruct::new(id, vector, payload);
        self.client.upsert_points_blocking(&self.collection_name, None, vec![point], None).await?;
        
        Ok(())
    }
}
