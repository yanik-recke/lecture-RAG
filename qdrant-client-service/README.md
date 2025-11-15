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

## Build & Run

**!** At the moment the path to the proto files in [build.rs](build.rs) needs
to be configured before building the Docker image. This is because
in the build process the proto directory gets moved to app/proto
and during development the proto files are in the root dir of
this repository **!**

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .compile_protos(&["proto/lecture_store.proto"], &["proto"])?;
    println!("Built!");
    Ok(())
}
```

The following environment variables are needed:

| Variable             | Description                       | Example              |
|----------------------|-----------------------------------|----------------------|
| `LECTURE_STORE_HOST` | Host address for the gRPC server  | `127.0.0.1`          |
| `LECTURE_STORE_PORT` | Port for the gRPC server          | `50051`              |
| `QDRANT_URL`         | URL of the Qdrant vector database | `http://qdrant:6334` |

```shell
# Create Docker network
docker network create lecture-rag-dev

# Run Qdrant locally
docker run -p 6333:6333 -p 6334:6334 --name qdrant \
    -v "$(pwd)/qdrant_storage:/qdrant/storage:z" --network lecture-rag-dev \
    qdrant/qdrant
    
# Build first (see above)
docker build -f qdrant-client-service/Dockerfile -t qdrant-client-service .

# Then run
docker run -p 40041:40041 --network lecture-rag-dev -e LECTURE_STORE_HOST=127.0.0.1 -e LECTURE_STORE_PORT=40041 -e QDRANT_URL=http://qdrant:6334 --rm qdrant-client-service
```