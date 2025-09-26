## Development

Flask server running on port `8001`.

Building docker image: `docker build -t "embeddinggemma".\n
Running container on Jetson Orin Nano: `docker run --gpus all --ipc=host -v ~/.cache/huggingface/hub:/root/.cache/huggingface/hub --ulimit memlock=-1 --ulimit stack=67108864 -it --rm -p 8001:8001  embeddinggemma`.

The run command caches the model on the Jetson Orin Nano by specifying the volume with *-v* flag.

Embedding endpoint can be tested via:
```
curl -X POST http://192.168.55.1:8001/embed \
    -H "Content-Type: application/json" \
    -d '{"text": "your text here"}'
```

Health endpoint: `curl 192.168.55.1:8001/health`