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

### Server

The gRPC server listens on port 50052.

## Torch

*torch* is not listed as a dependency in [pyproject.toml](pyproject.toml) as a specific
installation will be provided by the Nvidia container.

## Issues

[_] Implement the whisper service and the embedding service in the same
project so that the models can be loaded into memory on-demand to save
memory (as memory on the Jetson Orin Nano is limited)