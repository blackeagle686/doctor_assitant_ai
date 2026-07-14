pub mod routes;

use std::sync::Arc;
use crate::core::config::Config;
use crate::services::embedding::EmbeddingService;
use crate::brain::rag::qdrant_db::QdrantDb;
use crate::brain::llm::LlmClient;
use crate::brain::rag::redis_cache::RedisCache;
use routes::AppState;

pub async fn start_server() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    
    let embedding = Arc::new(EmbeddingService::new()?);
    let vdb = Arc::new(QdrantDb::new(&config).await?);
    let llm = Arc::new(LlmClient::new(&config)?);
    let redis_cache = Arc::new(RedisCache::new(&config)?);
    let config = Arc::new(config);
    
    let state = Arc::new(AppState {
        embedding,
        vdb,
        llm,
        redis_cache,
        config,
    });

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("API Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
