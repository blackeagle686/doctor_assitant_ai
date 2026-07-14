mod services;
mod api;
mod brain;
mod core;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("--- Testing Local STT ---");
    let transcript = brain::pipeline::run_recognition("./patient_record.wav", true).await;
    match transcript {
        Ok(text) => println!("STT Success! Transcript:\n{}", text),
        Err(e) => println!("STT Failed: {:?}", e),
    }
    
    println!("--- Tests Complete! Exiting before starting server... ---");
    return Ok(());
    
    println!("Starting Doctor Assistant API Server...");
    api::start_server().await?;
    
    Ok(())
}
