use crate::lecturestore::lecture_store_server::{LectureStore, LectureStoreServer};
use crate::lecturestore::{
    AddSummaryEmbeddingReq, AddSummaryEmbeddingRes, AddTranscriptEmbeddingReq,
    AddTranscriptEmbeddingRes, AddTranscriptEmbeddingSuccess, SummaryEmbedding, Timestamp,
    TranscriptEmbedding, add_transcript_embedding_res,
};
use anyhow::{Context, Result};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointId, PointStruct, UpsertPointsBuilder, Value,
    VectorParamsBuilder, point_id,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod lecturestore {
    tonic::include_proto!("lecturestore");
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = LectureStoreServerImpl::new(
        "127.0.0.1".to_string(),
        8080,
        "http://127.0.0.1:8081".to_string(),
    );

    server.start().await?;
    Ok(())
}

pub struct LectureStoreServerImpl {
    host: String,
    port: u32,
    qdrant_url: String,
}

impl LectureStoreServerImpl {
    pub fn new(host: String, port: u32, qdrant_url: String) -> LectureStoreServerImpl {
        LectureStoreServerImpl {
            host,
            port,
            qdrant_url,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .context("Could not create socket address")?;

        let client = Qdrant::from_url(&*self.qdrant_url).build()?;
        let service = LectureStoreService { client };

        Server::builder()
            .add_service(LectureStoreServer::new(service))
            .serve(addr)
            .await
            .context("Could not build server")?;
        Ok(())
    }
}

pub struct LectureStoreService {
    client: Qdrant,
}

#[tonic::async_trait]
impl LectureStore for LectureStoreService {
    async fn add_transcript_embedding(
        &self,
        request: Request<AddTranscriptEmbeddingReq>,
    ) -> std::result::Result<Response<AddTranscriptEmbeddingRes>, Status> {
        let transcript_embedding = request
            .into_inner()
            .transcript_embedding
            .ok_or_else(|| Status::invalid_argument("Field transcript_embedding is missing"))?;

        let collection_name = format!("{}_embedding", transcript_embedding.module);

        check_and_create_collection(&self.client, &*collection_name).await?;

        let new_uuid = Uuid::new_v4();
        let point_id = PointId {
            point_id_options: Some(point_id::PointIdOptions::Uuid(new_uuid.to_string().clone())),
        };

        let payload = build_transcript_payload(&transcript_embedding).map_err(|e| {
            Status::internal(format!("There was an error building the payload: {}", e))
        })?;

        let embedding_vector = transcript_embedding
            .embedding
            .ok_or_else(|| {
                Status::invalid_argument(
                    "Field embedding missing in TranscriptEmbedding from AddTranscriptEmbeddingReq",
                )
            })?
            .vector_data;

        self.client
            .upsert_points(
                UpsertPointsBuilder::new(
                    format!("{}_embedding", transcript_embedding.module),
                    vec![PointStruct::new(point_id, embedding_vector, payload)],
                )
                .wait(true),
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to upsert points: {}", e)))?;

        Ok(Response::new(AddTranscriptEmbeddingRes {
            result: Some(add_transcript_embedding_res::Result::SuccessPayload(
                AddTranscriptEmbeddingSuccess {
                    qdrant_point_id: new_uuid.to_string(),
                },
            )),
        }))
    }

    async fn add_summary_embedding(
        &self,
        request: Request<AddSummaryEmbeddingReq>,
    ) -> std::result::Result<Response<AddSummaryEmbeddingRes>, Status> {
        let summary_embedding = request
            .into_inner()
            .summary_embedding
            .ok_or_else(|| Status::invalid_argument("Missing summary_embedding field"))?;

        let collection_name = format!("{}_summary", summary_embedding.module);

        check_and_create_collection(&self.client, &*collection_name).await?;

        let payload = build_summary_payload(&summary_embedding);

        // Upsert embedding
        println!("{}", summary_embedding.lecture_name);
        todo!()
    }
}

/**
* Checks if a collection exists, if it does not exist,
* it will be created.
*/
async fn check_and_create_collection(client: &Qdrant, collection_name: &str) -> Result<(), Status> {
    match client.collection_exists(collection_name).await {
        Ok(false) => {
            // Collection doesn't exist, create it
            client
                .create_collection(
                    CreateCollectionBuilder::new(collection_name)
                        .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine)),
                )
                .await
                .map_err(|e| Status::internal(format!("Failed to create collection: {}", e)))?;
            Ok(())
        }
        Err(e) => {
            return Err(Status::internal(format!(
                "Failed to check if collection exists: {}",
                e
            )));
        }
        _ => Ok(()), // Do nothing as collection already exists
    }
}

fn build_transcript_payload(
    transcript_embedding: &TranscriptEmbedding,
) -> Result<HashMap<String, Value>> {
    let mut payload: HashMap<String, Value> = HashMap::new();
    let timestamp = transcript_embedding
        .timestamp
        .ok_or_else(|| Status::invalid_argument("Missing field timestamp"))?;

    payload.insert(
        "timestamp_start".to_string(),
        Value::from(timestamp.timestamp_start),
    );

    payload.insert(
        "timestamp_end".to_string(),
        Value::from(timestamp.timestamp_end),
    );

    payload.insert(
        "raw_content".to_string(),
        Value::from(&*transcript_embedding.raw_content),
    );

    payload.insert(
        "lecture_name".to_string(),
        Value::from(&*transcript_embedding.lecture_name),
    );

    Ok(payload)
}

fn build_summary_payload(summary_embedding: &SummaryEmbedding) -> HashMap<String, Value> {
    let mut payload: HashMap<String, Value> = HashMap::new();

    payload.insert(
        "raw_content".to_string(),
        Value::from(&*summary_embedding.raw_content),
    );

    payload.insert(
        "lecture_name".to_string(),
        Value::from(&*summary_embedding.lecture_name),
    );

    payload
}
