mod services;
mod api;
mod brain;
mod core;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Doctor Assistant API Server...");
    api::start_server().await?;
    
    Ok(())
}
