# Qdrant Client Service

## Build Docker Image

Build from root dir because it needs access to the /proto dir:

```
docker build -f qdrant-client-service/Dockerfile -t qdrant-client-service .
```

To build and push to [ghcr.io](https://ghcr.io):

```
docker buildx build -f qdrant-client-service/Dockerfile --platform linux/amd64 -t ghcr.io/yanik-recke/lecture-rag/qdrant-service:VERSION --push .
```

## Run

The following environment variables are needed:

| Variable             | Description                       | Example              |
|----------------------|-----------------------------------|----------------------|
| `LECTURE_STORE_HOST` | Host address for the gRPC server  | `127.0.0.1`          |
| `LECTURE_STORE_PORT` | Port for the gRPC server          | `50051`              |
| `QDRANT_URL`         | URL of the Qdrant vector database | `http://qdrant:6334` |