# 🩺 Doctor Assistant AI

<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/OpenAI-412991?style=for-the-badge&logo=openai&logoColor=white" alt="OpenAI" />
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker" />
  <img src="https://img.shields.io/badge/Axum-000000?style=for-the-badge" alt="Axum" />
</div>
<br>

**Doctor Assistant AI** is a high-performance, robust backend system built entirely in **Rust**. It is designed to act as an intelligent co-pilot for medical professionals. By capturing patient audio (e.g., during consultations or visits), the system transcribes the spoken words with extremely high accuracy—specifically optimized for complex dialects like **Egyptian Arabic mixed with English medical terminology**—and will ultimately generate comprehensive, structured AI medical reports to streamline doctors' workflows.

---

## 🌟 Key Features
- **Highly Accurate Speech-to-Text**: Optimized out-of-the-box for Egyptian Arabic and English code-switching using smart prompting.
- **Dual Recognition Engine**: 
  - **OpenAI API**: Blazing fast, cloud-based transcription using the Whisper-1 model.
  - **Local Whisper (whisper.cpp)**: Offline, privacy-first transcription using the `whisper-rs` engine. Automatically downloads the required `ggml` models.
- **Robust API Layer**: Built on `axum`, offering high-performance async endpoints for seamless integration with frontend or mobile applications.
- **Microservices Architecture**: Strictly adheres to software engineering best practices, including Strategy and Factory design patterns.
- **Docker Ready**: Fully containerized with a highly optimized multi-stage build, managing heavy C++ and ALSA audio dependencies automatically.

---

## 🏗️ Architecture & Layers

The project is thoughtfully divided into modular layers to ensure scalability and maintainability.

### 1. The API Layer (`src/api/`)
Acts as the bridge between the backend and external applications. It is powered by `axum` and `tokio`.
- **`POST /recognize`**: Accepts `multipart/form-data` containing audio files, streams them into the AI pipeline, and returns the accurate text transcript.
- **`GET /report`**: (WIP) Will return the fully structured AI medical report generated from the latest transcripts.

### 2. The Brain Layer (`src/brain/`)
The core orchestrator of the application.
- **`pipeline.rs`**: Acts as the nervous system. It takes incoming tasks (like an audio file), uses the Factory pattern to initialize the correct tool (Local vs. Cloud), and processes the data step-by-step.

### 3. The Services Layer (`src/services/`)
Contains all isolated functional modules.
- **`speech_recognition.rs`**: Defines the `SpeechRecognizer` trait and a `RecognizerFactory`, enabling seamless swapping between different AI models (Strategy Pattern).
- **`openai_audio/`**: Implementation of the `SpeechRecognizer` that communicates with OpenAI's API securely via `reqwest`.
- **`local_audio/`**: Implementation of the `SpeechRecognizer` that runs completely offline using `whisper-rs` (C++ bindings). 
- **`recorder.rs`**: Directly interfaces with host hardware (`cpal`) to manually capture high-quality `.wav` audio.

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.75+)
- An OpenAI API Key (if using the cloud engine)
- System dependencies (if building locally):
  - Ubuntu/Debian: `sudo apt install libasound2-dev cmake clang build-essential pkg-config libssl-dev`

### Setup
1. **Clone the repository**:
   ```bash
   git clone https://github.com/blackeagle686/doctor_assitant_ai.git
   cd doctor_assitant_ai
   ```

2. **Configure Environment Variables**:
   Create a `.env` file in the root directory:
   ```env
   OPENAI_API_KEY="sk-your-openai-api-key-here"
   ```

3. **Run the API Server Locally**:
   ```bash
   cargo run --release
   ```
   The server will start on `http://0.0.0.0:3000`.

---

## 🐳 Docker Deployment

We provide a streamlined Docker setup. It uses a **multi-stage build** to keep the final image minimal while compiling all the heavy C++ and ALSA drivers.

```bash
# 1. Build the image
docker build -t doctor_assist .

# 2. Run the container (Mapping the sound device is required for manual recording)
docker run -p 3000:3000 --device /dev/snd --env-file .env doctor_assist
```

---

## 🗺️ Roadmap

- [x] **Phase 1: Foundation & Audio Capture**
  - Setup Rust environment and `cpal` hardware audio capture.
- [x] **Phase 2: Transcription Engine (The "Ears")**
  - Implement Factory/Strategy patterns for AI models.
  - Integrate OpenAI Whisper API (Cloud).
  - Integrate `whisper-rs` (Local Offline).
  - Optimize for Egyptian Arabic + English medical terms.
- [x] **Phase 3: Network & API**
  - Build the `axum` async REST API to allow external apps to upload audio.
  - Implement the `/recognize` route.
- [ ] **Phase 4: Intelligence & Reporting (The "Brain")**
  - Connect the transcription output to an LLM (e.g., GPT-4 or a local LLaMA model).
  - Parse the raw text and generate structured, professional medical reports.
  - Finalize the `/report` endpoint.
- [ ] **Phase 5: Frontend Integration**
  - Build a sleek, modern web/mobile frontend for doctors to interact with the assistant seamlessly.

---
*Built with passion to empower healthcare professionals with cutting-edge AI.*
