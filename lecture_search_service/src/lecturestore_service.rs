use crate::lecturestore::lecture_store_client::LectureStoreClient;
use crate::lecturestore::{
    AddTranscriptEmbeddingReq, AddTranscriptEmbeddingRes, SimilaritySearchReq, SimilaritySearchRes,
    Timestamp, TranscriptEmbedding, VectorEmbedding,
};
use anyhow::{Context, Result};
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

pub struct LectureStoreService {
    client: Arc<Mutex<LectureStoreClient<Channel>>>,
}

impl LectureStoreService {
    pub async fn new(host: String, port: u32) -> Result<Self> {
        let endpoint =
            Endpoint::new(format!("{}:{}", host, port)).context("Could not create endpoint")?;
        let channel = endpoint
            .connect()
            .await
            .context("Failed to connect to server")?;

        let client = LectureStoreClient::new(channel)
            .max_decoding_message_size(100 * 1024 * 1024) // 100 MB
            .max_encoding_message_size(100 * 1024 * 1024); // 100 MB

        Ok(LectureStoreService {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn add_transcript_embedding(
        &self,
        module: String,
        timestamp: Timestamp,
        raw_content: String,
        lecture_name: String,
        embedding: VectorEmbedding,
    ) -> Result<AddTranscriptEmbeddingRes> {
        let trans_embedding = Request::new(AddTranscriptEmbeddingReq {
            transcript_embedding: Some(TranscriptEmbedding {
                module,
                timestamp: Some(timestamp),
                raw_content,
                lecture_name,
                embedding: Some(embedding),
            }),
        });

        Ok(self
            .client
            .lock()
            .await
            .add_transcript_embedding(trans_embedding)
            .await
            .map_err(|e| {
                error!("Error while trying to add embedding: {}", e);
                Status::internal("Error while trying to add embedding")
            })?
            .into_inner())
    }

    pub async fn add_summary_embedding() -> Result<(), Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    pub async fn perform_similarity_search(
        &self,
        module: String,
        embedding: VectorEmbedding,
    ) -> Result<SimilaritySearchRes> {
        let req = Request::new(SimilaritySearchReq {
            embedding: Some(embedding),
            module,
        });

        Ok(self
            .client
            .lock()
            .await
            .similarity_search(req)
            .await
            .context("Could not perform similarity search")?
            .into_inner())
    }
}
