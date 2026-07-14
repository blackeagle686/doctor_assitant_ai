mod services;
mod api;
mod brain;
mod core;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running Component Tests...");

    let config = core::config::Config::from_env()?;
    
    println!("--- Testing STT ---");
    // Testing STT with local = false (OpenAI by default for speed, unless you prefer local)
    // We'll test the Recognizer directly via run_recognition
    let transcript = brain::pipeline::run_recognition("./patient_record.wav", false).await.unwrap_or_else(|e| {
        println!("STT Failed: {:?}", e);
        "Fallback text for testing embedding: Patient has a mild headache and fever.".to_string()
    });
    println!("Transcript:\n{}\n", transcript);

    println!("--- Testing Embedding ---");
    let embedder = services::embedding::EmbeddingService::new()?;
    let query_vector = embedder.embed(&transcript)?;
    println!("Embedded vector size: {}\n", query_vector.len());

    println!("--- Testing Qdrant DB ---");
    let qdrant = brain::rag::qdrant_db::QdrantDb::new(&config).await?;
    println!("Inserting vector into Qdrant...");
    match qdrant.insert(999, query_vector.clone(), &transcript).await {
        Ok(_) => println!("Insert successful."),
        Err(e) => println!("Insert failed: {:?}", e),
    }
    
    println!("Searching vector in Qdrant...");
    match qdrant.search(query_vector, 1).await {
        Ok(results) => println!("Search results:\n{:?}\n", results),
        Err(e) => println!("Search failed: {:?}", e),
    }
    
    println!("--- Tests Complete! Exiting before starting server... ---");
    return Ok(());
    
    println!("Starting Doctor Assistant API Server...");
    api::start_server().await?;
    
    Ok(())
}
