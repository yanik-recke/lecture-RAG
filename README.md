# Workflow

1. Upload lecture
2. Lecture gets transcribed
3. Create vector embeddings of transcriptions
4. RAG

# Techstack
Frontend: Svelte
Speech-To-Text: [primeline/whisper-large-v3-turbo-german](https://huggingface.co/primeline/whisper-large-v3-german)

Vector Store: milvus

Embedding: [google/embeddinggemma-300m](https://huggingface.co/google/embeddinggemma-300m)

Retrieval Service: Spring AI

# Hardware
[Nvidia Jetson Orin Nano](https://www.nvidia.com/de-de/autonomous-machines/embedded-systems/jetson-orin/nano-super-developer-kit/)
