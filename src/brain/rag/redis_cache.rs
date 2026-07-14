use anyhow::{Context, Result};
use redis::AsyncCommands;
use std::env;

pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(config: &crate::core::config::Config) -> Result<Self> {
        let uri = config.redis_url.clone();
        println!("Connecting to Redis Cache at {}", uri);
        let client = redis::Client::open(uri).context("Failed to connect to Redis")?;
        
        Ok(Self { client })
    }

    /// CAG Layer: Store the final generated report using a hash of the transcript as the key
    pub async fn cache_report(&self, transcript_hash: &str, report: &str) -> Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        // Set with an expiration of 24 hours (86400 seconds)
        let _: () = con.set_ex(format!("report:{}", transcript_hash), report, 86400).await?;
        Ok(())
    }

    /// CAG Layer: Retrieve a cached report
    pub async fn get_cached_report(&self, transcript_hash: &str) -> Result<Option<String>> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = con.get(format!("report:{}", transcript_hash)).await?;
        Ok(result)
    }
}
