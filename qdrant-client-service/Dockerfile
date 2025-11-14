FROM lukemathwalker/cargo-chef:latest-rust-1.91.1-alpine3.21 AS chef

LABEL org.opencontainers.image.authors="Yanik Recke <recke.yanik@outlook.de>"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.description="Image for the Qdrant client service"

FROM chef AS planner
WORKDIR /app
COPY ./qdrant-client-service .
COPY ./proto ./proto
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
RUN apk add --no-cache protobuf-dev
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./qdrant-client-service .
COPY ./proto ./proto
RUN cargo build --release --bin qdrant-client-service

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/qdrant-client-service /usr/local/bin
ENTRYPOINT ["/usr/local/bin/qdrant-client-service"]