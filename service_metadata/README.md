# Metadata DB Service

This service communicates with the database (MongoDB instance) that
holds all the metadata to the modules and lectures. The
message types and procedures are defined in its respective
[proto file](../proto/metadata_service.proto).

## Development

### Docker

Build locally from root dir (one dir above project dir):

```shell
# Build
docker buildx build -f service_metadata/Dockerfile --platform l
inux/amd64 -t metadata-service .

# Run 
docker run -p 40042:40042 --network lecture-rag-dev -e RUST_LOG=DEBUG \
-e METADATA_SERVICE_HOST=0.0.0.0 \
-e METADATA_SERVICE_PORT=40042 \
-e METADATA_DB_URI=mongodb://localhost:27017/ \
-e METADATA_DB_NAME=lecture_metadata \
-e METADATA_LECTURE_NAME=lectures \
-e METADATA_MODULE_NAME=modules \
--rm \
-d \
metadata-service:latest
```

### Tests

To run tests: `cargo test`  
Some tests do require external services (like the database) to be running.