FROM lukemathwalker/cargo-chef:latest-rust-1.91.1-alpine3.21 AS chef

LABEL org.opencontainers.image.authors="Yanik Recke <recke.yanik@outlook.de>"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.description="Image for the lecture search service"

FROM chef AS planner
WORKDIR /app
COPY ./lecture_search_service .
COPY ./proto ./proto
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
RUN apk add --no-cache protobuf-dev
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./lecture_search_service .
COPY ./proto ./proto
RUN cargo build --release --bin lecture_search_service

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/lecture_search_service /usr/local/bin
ENTRYPOINT ["/usr/local/bin/lecture_search_service"]