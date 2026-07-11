# Stage 1: Builder
FROM rust:1.94-slim-bookworm AS builder

# Install system dependencies required for compiling the project.
# - pkg-config, libssl-dev: required for reqwest (OpenSSL)
# - libasound2-dev: required for cpal (audio recording)
# - cmake, clang, build-essential: required for whisper-rs (compiling whisper.cpp)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    cmake \
    clang \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/doctor_assist

# Copy over the source code
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies needed to run the binary.
# - libasound2, alsa-utils: ALSA sound drivers for audio recording
# - libssl3, ca-certificates: HTTPS capabilities for OpenAI API and downloading models
RUN apt-get update && apt-get install -y \
    libasound2 \
    alsa-utils \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/doctor_assist/target/release/doctor_assist /usr/local/bin/doctor_assist

# Set the environment variable to ensure logs or prompts are visible
ENV RUST_BACKTRACE=1

# Command to run the application
CMD ["doctor_assist"]
