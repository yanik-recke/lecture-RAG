## Development

Generate the messages and stubs with (you might have to create the target/ directory):

```shell
  python3 -m grpc_tools.protoc \
  -I../proto \
  --python_out=./target \
  --grpc_python_out=./target \
  --pyi_out=./target \
  ../proto/embedding_service.proto \
  ../proto/lecture_store.proto
```

Flask server running on port `8001`.

Building docker image: `docker build -t "embeddinggemma".\n
Running container on Jetson Orin Nano: `docker run --gpus all --ipc=host -v ~/.cache/huggingface/hub:
/root/.cache/huggingface/hub --ulimit memlock=-1 --ulimit stack=67108864 -it --rm -p 8001:8001 embeddinggemma`.

The run command caches the model on the Jetson Orin Nano by specifying the volume with *-v* flag.

Embedding endpoint can be tested via:

```
curl -X POST http://192.168.55.1:8001/embed \
    -H "Content-Type: application/json" \
    -d '{"text": "your text here"}'
```

Health endpoint: `curl 192.168.55.1:8001/health`

## Issues

[_] Implement the whisper service and the embedding service in the same
project so that the models can be loaded into memory on-demand to save
memory (as memory on the Jetson Orin Nano is limited)