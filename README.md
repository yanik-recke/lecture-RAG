# Lecture RAG

A microservices-based Retrieval-Augmented Generation (RAG) system for lecture content, designed to transcribe, embed, and enable semantic search across lecture recordings. Originally optimized for the NVIDIA Jetson Orin Nano edge device.

## Overview

This system enables users to upload lecture recordings, automatically transcribe them using speech-to-text, generate vector embeddings, and perform semantic search using RAG. The architecture follows a microservices pattern with gRPC communication between services.

## Architecture

### Services

- **UI** - Next.js frontend for uploading lectures and querying content
- **BFF** - Spring Boot backend-for-frontend service that orchestrates communication between microservices
- **Lecture Search Service** - Rust service that coordinates transcription, embedding, and storage operations
- **Qdrant Client Service** - Rust service managing vector database operations
- **Service Embedding** - Python service generating vector embeddings using sentence transformers
- **Service Speech-to-Text** - Python/Flask service for transcribing audio using Whisper
- **Qdrant** - Vector database for storing and querying embeddings

### Technology Stack

**Frontend**
- Next.js 16 with React 19
- TypeScript
- Tailwind CSS
- Radix UI components

**Backend Services**
- BFF: Java 17 with Spring Boot 3.5 and Spring gRPC
- Lecture Search Service: Rust with Tonic (gRPC)
- Qdrant Client Service: Rust with Tonic (gRPC)

**ML Services**
- Speech-to-Text: [primeline/whisper-large-v3-turbo-german](https://huggingface.co/primeline/whisper-large-v3-german)
- Embedding: [google/embeddinggemma-300m](https://huggingface.co/google/embeddinggemma-300m) via sentence-transformers

**Infrastructure**
- Vector Store: Qdrant
- Communication: gRPC with Protocol Buffers
- Containerization: Docker & Docker Compose
- CI/CD: Jenkins

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Hugging Face token (for accessing the embedding model)

### Running the Services

1. Add your Hugging Face token to the Docker Compose file for the embedding service

2. Start all services:
   ```bash
   docker compose up
   ```

The services will be available at:
- UI: Check docker-compose.yml for port mappings
- BFF: Port 40999
- Lecture Search Service: Port 40998
- Qdrant Dashboard: Port 6333
- Qdrant gRPC: Port 6334

### Development

The repository follows a monorepo structure. Each service has its own build configuration:

**UI**
```bash
cd ui
npm install
npm run dev
```

**BFF**
```bash
cd bff
./mvnw spring-boot:run
```

**Rust Services**
```bash
cd lecture_search_service  # or qdrant-client-service
cargo run
```

**Python Services**
```bash
cd service_embedding  # or service_speechtotext
pip install -r requirements.txt
python main.py
```

## Hardware Considerations

The ML inference services (Whisper speech-to-text and embedding generation) were specifically optimized to run on the [NVIDIA Jetson Orin Nano](https://www.nvidia.com/de-de/autonomous-machines/embedded-systems/jetson-orin/nano-super-developer-kit/) with 8GB RAM. The other services (UI, BFF, Rust services, Qdrant) can run on standard hardware without special considerations.

Running both the embedding and Whisper models simultaneously on the Jetson Nano can cause memory pressure due to the limited 8GB RAM shared between CPU and GPU.

### Jetson Nano Optimization Tips

When running the ML services on Jetson Nano:
- Monitor GPU/CPU memory usage: `tegrastats`
- Monitor system memory: `watch -n 0.5 free -m`
- Check for OOM errors: `sudo dmesg | tail -20`

If experiencing out-of-memory issues, consider increasing swap space. The Jetson Nano's iGPU shares memory with the CPU, so both GPU and system memory need to be monitored together.

## Workflow

1. Upload lecture recording through the UI
2. Lecture gets transcribed using Whisper
3. Transcription is chunked and embedded using the embedding service
4. Vector embeddings are stored in Qdrant
5. Users can query lectures using semantic search via RAG

## CI/CD

The project uses Jenkins for continuous integration with:
- Automated Docker image builds
- Version management
- Multi-service orchestration
- Container registry publishing (GHCR)

## Project Structure

```
lecture-RAG/
├── ui/                          # Next.js frontend
├── bff/                         # Spring Boot BFF service
├── lecture_search_service/      # Rust lecture coordination service
├── qdrant-client-service/       # Rust Qdrant client wrapper
├── service_embedding/           # Python embedding service
├── service_speechtotext/        # Python Whisper service
├── openai_speechtotext/         # OpenAI-based transcription (legacy)
├── openai_summarize/            # OpenAI-based summarization (legacy)
├── proto/                       # Protocol Buffer definitions
├── jenkins/                     # Jenkins pipeline scripts
├── docker-compose.yml           # Main orchestration
└── docker-compose.jetson.yml    # Jetson-specific config
```

## License

See individual service directories for license information.

## Author

Yanik Luis Recke - recke.yanik@outlook.de
