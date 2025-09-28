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


# How to run

Make sure to include the hugging face token in the Docker Compose file to get access to the embedding model.

The compose file is configured for usage on the Orin Jetson Nano device, as is the embedding server. 

`docker compose up`

I had to increase the swap size because I kept getting OOM errors when running all services together. This was probably due to the Nano only having 8GB RAM and having both the embedding and whisper model loaded into memory was too much.

I used [this](https://www.forecr.io/blogs/programming/how-to-increase-swap-space-on-jetson-modules?srsltid=AfmBOorOXqisLFP2wG-SUj2HCyJ6K6CxMbueJKwOCevnfre_NqRJ6RN_) guide and increased it by 6 GB.

Since the Nano has an iGPU and shares the memory with the CPU the GPU usage can be monitored via `tegrastats`. Also helpful: `free -h` or rather `watch -n 0.5 free -m`.

If any container quits silently inspecting the kernel messages might help identifying the cause: `sudo dmesg | tail -20` (It will probably be an OOM error). 