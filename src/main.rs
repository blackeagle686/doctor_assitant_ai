mod services;
mod api;
mod brain;

use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Doctor Assistant API Server...");
    api::start_server().await?;
    
    Ok(())
}
