# Lecture Search Service

This is the main facilitator. It chains
the different calls to the other microservices
and handles parsing of the results of
each respective call.

## Structure

Each service contains the gRPC client implementations
for each microservice. The clients / services
are created in [main.rs](src/main.rs). Their implementation
almost makes it look like the function calls are local
(as intended by gRPC).

## Development

### Docker

Build locally from root dir (one dir above):

```shell
# Build
docker build -f lecture_search_service/Dockerfile -t lecture-search-service .

#Run
docker run -p 40998:40998 --network lecture-rag-dev -e RUST_LOG=DEBUG \
-e LECTURE_SEARCH_HOST=0.0.0.0 \
-e LECTURE_SEARCH_PORT=40998 \
-e LECTURESTORE_HOST=http://qdrantclient \
-e LECTURESTORE_PORT=40041 \
-e WHISPER_HOST=http://192.168.178.54 \
-e WHISPER_PORT=50051 \
-e EMBEDDING_HOST=http://192.168.178.54 \
-e EMBEDDING_PORT=50052 \
--rm \
lecture-search-service:latest
```

## Testing

Each respective microservice application
should contain their own tests but since this
service depends on **all** the other services,
this project includes integration tests. In
the future a *docker-compose* file will be
provided to build and start each of
the services locally. At the moment
testing is neglected and almost completely missing.
I might have to rewrite some of the services
to follow a test driven development kind of approach.
If each of the microservice passed their
component tests, one could almost limit tests
for this application as an integration test as
that is pretty much all that is done here.

## Roadmap

[x] Transcription path   
[_] Search path  
[_] Tests