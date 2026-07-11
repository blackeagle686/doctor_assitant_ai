mod services;
mod api;
mod brain;

use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Doctor Assistant Recorder...");
    
    let mut recorder = services::recorder::Recorder::new();
    let filename = "patient_record.wav";
    
    println!("Recording patient's audio to {}...", filename);
    recorder.start_recording(filename)?;
    
    println!("Please speak now... (recording for 5 seconds)");
    sleep(Duration::from_secs(5));
    
    recorder.stop_recording()?;
    println!("Recording stopped and saved to {}.", filename);
    
    println!("--------------------------------------------------");
    println!("Step 2: Recognizing speech...");
    
    // Set to `true` to use the Local Whisper model, or `false` to use OpenAI.
    let use_local_model = false; 
    
    match brain::pipeline::run_recognition(filename, use_local_model).await {
        Ok(text) => {
            println!("Recognition successful!");
            println!("Transcript:");
            println!("\"{}\"", text);
        }
        Err(e) => {
            eprintln!("Failed to recognize speech: {:?}", e);
        }
    }
    
    Ok(())
}
