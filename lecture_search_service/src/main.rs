mod embedding_service;
mod lecturestore_service;
mod whisper_service;

use crate::embedding_service::EmbeddingService;
use crate::lectureservice::lecture_service_server::LectureService;
use crate::lectureservice::{SearchReq, SearchRes, TranscribeReq, TranscribeRes};
use crate::lecturestore_service::LectureStoreService;
use crate::whisper_service::WhisperService;
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

// #[tonic::async_trait]
fn main() {
    env_logger::init();
    println!("Hello, world!");
}

struct LectureSearchService {
    embedding_service: EmbeddingService,
    whisper_service: WhisperService,
    lecturestore_service: LectureStoreService,
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

            self.lecturestore_service
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
