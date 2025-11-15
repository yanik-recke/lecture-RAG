mod embedding_service;
mod lecturestore_service;
mod whisper_service;

use crate::embedding_service::EmbeddingService;
use crate::lectureservice::lecture_service_server::{LectureService, LectureServiceServer};
use crate::lectureservice::{SearchReq, SearchRes, TranscribeReq, TranscribeRes};
use crate::lecturestore_service::LectureStoreService;
use crate::whisper_service::WhisperService;
use anyhow::{Context, Result};
use log::info;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod lectureservice {
    tonic::include_proto!("lectureservice");
}

pub mod lecturestore {
    tonic::include_proto!("lecturestore");
}

pub mod embeddingservice {
    tonic::include_proto!("embeddingservice");
}

pub mod completionservice {
    tonic::include_proto!("completionservice");
}

pub mod summaryservice {
    tonic::include_proto!("summaryservice");
}

pub mod whisperservice {
    tonic::include_proto!("whisperservice");
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let whisper_host = std::env::var("WHISPER_HOST").context("WHISPER_HOST must be set")?;
    let whisper_port = std::env::var("WHISPER_PORT")
        .context("WHISPER_PORT must be set")?
        .parse::<u32>()
        .context("WHISPER_PORT must be a valid number")?;

    let embedding_host = std::env::var("EMBEDDING_HOST").context("EMBEDDING_HOST must be set")?;
    let embedding_port = std::env::var("EMBEDDING_PORT")
        .context("EMBEDDING_PORT must be set")?
        .parse::<u32>()
        .context("EMBEDDING_PORT must be a valid number")?;

    let lecturestore_host =
        std::env::var("LECTURESTORE_HOST").context("LECTURESTORE_HOST must be set")?;
    let lecturestore_port = std::env::var("LECTURESTORE_PORT")
        .context("LECTURESTORE_PORT must be set")?
        .parse::<u32>()
        .context("LECTURESTORE_PORT must be a valid number")?;

    let lecture_search_host =
        std::env::var("LECTURE_SEARCH_HOST").context("LECTURE_SEARCH_HOST must be set")?;
    let lecture_search_port = std::env::var("LECTURE_SEARCH_PORT")
        .context("LECTURE_SEARCH_PORT must be set")?
        .parse::<u32>()
        .context("LECTURE_SEARCH_PORT must be a valid number")?;

    let server = LectureSearchServerImpl::new(
        whisper_host,
        whisper_port,
        embedding_host,
        embedding_port,
        lecturestore_host,
        lecturestore_port,
        lecture_search_host,
        lecture_search_port,
    )
    .await
    .context("Failed to create new server")?;

    server.start().await.context("Server failed")?;
    Ok(())
}

struct LectureSearchServerImpl {
    service: LectureSearchService,
    host: String,
    port: u32,
}

impl LectureSearchServerImpl {
    pub async fn new(
        whisper_host: String,
        whisper_port: u32,
        embedding_host: String,
        embedding_port: u32,
        lecturestore_host: String,
        lecturestore_port: u32,
        host: String,
        port: u32,
    ) -> Result<Self> {
        info!("Creating the services");

        let whisper_service = WhisperService::new(whisper_host, whisper_port)
            .await
            .context("Failed to create whisper service")?;

        let embedding_service = EmbeddingService::new(embedding_host, embedding_port)
            .await
            .context("Failed to create embedding service")?;

        let lecturestore_service = LectureStoreService::new(lecturestore_host, lecturestore_port)
            .await
            .context("Failed to create lecture store service")?;

        info!("Services created successfully");
        let service =
            LectureSearchService::new(embedding_service, whisper_service, lecturestore_service);

        Ok(Self {
            host,
            port,
            service,
        })
    }

    pub async fn start(self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .context("Could not create socket address")?;

        info!("Starting server on {}:{}", self.host, self.port);
        Server::builder()
            .add_service(LectureServiceServer::new(self.service))
            .serve(addr)
            .await
            .context("Could not build server")?;
        Ok(())
    }
}

struct LectureSearchService {
    embedding_service: EmbeddingService,
    whisper_service: WhisperService,
    lecture_store_service: LectureStoreService,
}

impl LectureSearchService {
    pub fn new(
        embedding_service: EmbeddingService,
        whisper_service: WhisperService,
        lecture_store_service: LectureStoreService,
    ) -> Self {
        LectureSearchService {
            embedding_service,
            whisper_service,
            lecture_store_service,
        }
    }
}

#[tonic::async_trait]
impl LectureService for LectureSearchService {
    async fn transcribe(
        &self,
        request: Request<TranscribeReq>,
    ) -> Result<Response<TranscribeRes>, Status> {
        let transcribe_req = request.into_inner();

        let req = TranscribeReq {
            file_payload: Some(
                transcribe_req
                    .file_payload
                    .ok_or_else(|| Status::invalid_argument("Missing file payload"))?,
            ),
            lecture_name: transcribe_req.lecture_name.clone(),
            module: transcribe_req.module.clone(),
        };

        let res = self.whisper_service.transcribe(req).await.map_err(|e| {
            Status::internal(format!(
                "Sending request to whisper service resulted in error: {}",
                e
            ))
        })?;

        // TODO call summary service async

        let res = res
            .payload
            .ok_or_else(|| Status::aborted("Transcription failed"))?;

        for chunk in res.chunks {
            let embedding_res = self
                .embedding_service
                .embed(chunk.text.clone())
                .await
                .map_err(|e| Status::internal(format!("Failed to embed error: {}", e)))?;

            let embedding = match embedding_res.result {
                Some(embeddingservice::new_embedding_res::Result::Vec(vec)) => vec,
                Some(embeddingservice::new_embedding_res::Result::ErrorMsg(err)) => {
                    return Err(Status::internal(format!("Embedding failed: {}", err)));
                }
                None => {
                    return Err(Status::internal("Embedding service returned empty result"));
                }
            };

            self.lecture_store_service
                .add_transcript_embedding(
                    transcribe_req.module.clone(),
                    chunk
                        .timestamp
                        .ok_or_else(|| Status::internal("Chunk was missing timestamp"))?,
                    chunk.text.clone(),
                    transcribe_req.lecture_name.clone(),
                    embedding,
                )
                .await
                .map_err(|e| {
                    Status::internal(format!(
                        "Error while trying to add embedding of chunk: {}",
                        e
                    ))
                })?;
        }

        Ok(Response::new(TranscribeRes {
            error_message: None,
        }))
    }

    async fn search(&self, request: Request<SearchReq>) -> Result<Response<SearchRes>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}
