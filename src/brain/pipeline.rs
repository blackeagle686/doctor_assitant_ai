use anyhow::Result;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::services::speech_recognition::{RecognizerFactory, RecognizerType};

use crate::api::routes::AppState;
use std::sync::Arc;

pub async fn run_recognition(file_path: &str, use_local: bool) -> Result<String> {
    let rec_type = if use_local {
        RecognizerType::Local
    } else {
        RecognizerType::OpenAI
    };

    println!("Initializing speech recognizer...");
    let recognizer = RecognizerFactory::create(rec_type).await?;

    println!("Starting recognition pipeline step...");
    let transcript = recognizer.recognize(file_path).await?;
    
    Ok(transcript)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub async fn generate_report(transcript: &str, state: Arc<AppState>) -> Result<String> {
    println!("Step 1: Checking Redis CAG (Cache-Augmented Generation) layer...");
    let transcript_hash = calculate_hash(&transcript).to_string();
    
    if let Ok(Some(cached_report)) = state.redis_cache.get_cached_report(&transcript_hash).await {
        println!("Cache hit! Returning pre-generated report from Redis.");
        return Ok(cached_report);
    }
    println!("Cache miss. Proceeding to generate report...");

    println!("Step 2: Correcting & Summarizing raw transcript via LLM...");
    
    let correction_prompt = "You are a medical AI assistant. Your task is to clean up, correct grammar, and summarize the following messy speech-to-text transcript from a patient consultation. The audio contains Egyptian Arabic and English medical terms. Output only the clean, accurate English summary of the medical situation.";
    let cleaned_transcript = state.llm.complete(correction_prompt, transcript).await?;
    println!("Cleaned Transcript: {}", cleaned_transcript);

    println!("Step 3: Embedding text using local all-MiniLM-L6-v2...");
    let query_vector = state.embedding.embed(&cleaned_transcript)?;

    println!("Step 4: RAG Retrieval from Qdrant Vector Database...");
    // We attempt to find 3 relevant medical guidelines or similar past cases
    let retrieved_context = state.vdb.search(query_vector, 3).await.unwrap_or_else(|_| {
        println!("Qdrant search failed or empty, proceeding without RAG context.");
        vec![]
    });

    println!("Step 5: Generating final accurate report via LLM...");
    let context_str = retrieved_context.join("\n\n");
    let report_prompt = format!(
        "You are an expert Doctor Assistant AI. Create a highly accurate, structured medical report for the doctor based on the patient's summarized transcript.\n\n\
        Retrieved Medical Knowledge / Guidelines context:\n{}\n\n\
        Patient Transcript:\n{}\n\n\
        Output a structured medical report with sections: Patient Summary, Identified Symptoms, Possible Conditions, and Recommended Next Steps.",
        if context_str.is_empty() { "None" } else { &context_str },
        cleaned_transcript
    );

    let final_report = state.llm.complete("You are an expert medical AI report generator. Output the final report cleanly.", &report_prompt).await?;

    println!("Step 6: Caching the final report in Redis CAG layer...");
    let _ = state.redis_cache.cache_report(&transcript_hash, &final_report).await;

    Ok(final_report)
}
