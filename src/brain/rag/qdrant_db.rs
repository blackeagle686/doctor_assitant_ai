use anyhow::{Context, Result};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{PointStruct, SearchPoints};
use std::env;

pub struct QdrantDb {
    client: Qdrant,
    collection_name: String,
}

impl QdrantDb {
    pub async fn new() -> Result<Self> {
        let uri = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());
        let collection_name = "medical_knowledge".to_string();
        
        println!("Connecting to Qdrant at {}", uri);
        let client = Qdrant::from_url(&uri).build()?;

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

        let response = self.client.search_points(search_request).await.context("Failed to search Qdrant")?;
        
        let mut results = Vec::new();
        for point in response.result {
            if let Some(qdrant_client::qdrant::value::Kind::StringValue(text)) = point.payload.get("text").and_then(|v| v.kind.as_ref()) {
                results.push(text.to_string());
            }
        }

        Ok(results)
    }

    pub async fn insert(&self, id: u64, vector: Vec<f32>, text: &str) -> Result<()> {
        let payload: qdrant_client::qdrant::Payload = serde_json::json!({"text": text}).try_into().unwrap_or_default();
        
        let point = PointStruct::new(id, vector, payload);
        let request = qdrant_client::qdrant::UpsertPointsBuilder::new(&self.collection_name, vec![point]).wait(true);
        self.client.upsert_points(request).await?;
        
        Ok(())
    }
}
