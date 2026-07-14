use axum::{
    extract::Multipart,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tower_http::cors::{Any, CorsLayer};

use crate::brain::pipeline;

#[derive(Serialize)]
pub struct RecognizeResponse {
    pub transcript: String,
}

#[derive(Deserialize)]
pub struct ReportRequest {
    pub transcript: String,
}

#[derive(Serialize)]
pub struct ReportResponse {
    pub report: String,
}

pub fn create_router() -> Router {
    // Basic CORS setup to allow other applications to connect easily
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/recognize", post(recognize_handler))
        .route("/report", post(report_handler))
        .layer(cors)
}


pub struct

/// Endpoint: POST /recognize
/// Accepts a multipart form data with a file field (e.g., "audio" or "file").
/// Returns the transcribed text.
async fn recognize_handler(mut multipart: Multipart) -> Result<Json<RecognizeResponse>, String> {
    println!("Received /recognize request");
    let mut temp_file_path = String::new();
    
    // Extract the uploaded file from the multipart request
    while let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let name = field.name().unwrap_or("").to_string();
        if name == "audio" || name == "file" {
            let data = field.bytes().await.map_err(|e| e.to_string())?;
            
            // Save it to a temporary file for the pipeline to process
            let temp_path = format!("temp_upload_{}.wav", uuid::Uuid::new_v4());
            let mut file = File::create(&temp_path).await.map_err(|e| e.to_string())?;
            file.write_all(&data).await.map_err(|e| e.to_string())?;
            
            temp_file_path = temp_path;
            break;
        }
    }

    if temp_file_path.is_empty() {
        return Err("No audio file uploaded. Please send a file in the 'audio' or 'file' field.".to_string());
    }

    // Call our speech recognition pipeline!
    // Using OpenAI by default because it's fast and highly accurate for Egyptian Arabic.
    let use_local_model = false; 
    let result = pipeline::run_recognition(&temp_file_path, use_local_model).await;

    // Clean up the temporary uploaded file
    let _ = tokio::fs::remove_file(&temp_file_path).await;

    let transcript = result.map_err(|e| format!("Recognition failed: {:?}", e))?;

    Ok(Json(RecognizeResponse { transcript }))
}

/// Endpoint: POST /report
/// Generates the final AI generated medical report from a transcript.
async fn report_handler(Json(payload): Json<ReportRequest>) -> Result<Json<ReportResponse>, String> {
    println!("Received /report request");
    
    let report = pipeline::generate_report(&payload.transcript)
        .await
        .map_err(|e| format!("Failed to generate report: {:?}", e))?;
        
    Ok(Json(ReportResponse { report }))
}
