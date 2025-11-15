use crate::lecturestore::lecture_store_server::{LectureStore, LectureStoreServer};
use crate::lecturestore::{
    AddSummaryEmbeddingReq, AddSummaryEmbeddingRes, AddTranscriptEmbeddingReq,
    AddTranscriptEmbeddingRes, AddTranscriptEmbeddingSuccess, SimilaritySearchReq,
    SimilaritySearchRes, SummaryEmbedding, Timestamp, TranscriptEmbedding,
    add_transcript_embedding_res,
};
use anyhow::{Context, Result};
use log::{debug, error, info};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointId, PointStruct, Query, QueryPointsBuilder,
    UpsertPointsBuilder, Value, VectorParamsBuilder, point_id,
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
    env_logger::init();
    let host = std::env::var("LECTURE_STORE_HOST")
        .context("LECTURE_STORE_HOST environment variable must be set")?;

    let port = std::env::var("LECTURE_STORE_PORT")
        .expect("LECTURE_STORE_PORT environment variable must be set")
        .parse::<u32>()
        .context("LECTURE_STORE_PORT must be a valid number")?;

    let qdrant_url =
        std::env::var("QDRANT_URL").context("QDRANT_URL environment variable must be set")?;

    info!(
        "Configuring server with host: {}, port: {} and qdrant endpoint: {}",
        host, port, qdrant_url
    );
    let server = LectureStoreServerImpl::new(host, port, qdrant_url);

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

        let service = LectureStoreService::new(client);

        debug!("Starting server on {}:{}", self.host, self.port);

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

impl LectureStoreService {
    pub fn new(client: Qdrant) -> Self {
        LectureStoreService { client }
    }
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

        debug!(
            "Received AddTranscriptEmbeddingReq with module: {}, raw_content: {}, \
        lecture_name: {}",
            transcript_embedding.module,
            transcript_embedding.raw_content,
            transcript_embedding.lecture_name
        );

        let collection_name = format!("{}_embedding", transcript_embedding.module);

        check_and_create_collection(&self.client, &*collection_name).await?;

        let new_uuid = Uuid::new_v4();
        let point_id = PointId {
            point_id_options: Some(point_id::PointIdOptions::Uuid(new_uuid.to_string().clone())),
        };

        debug!("Generated new_uuid: {}", new_uuid);

        let payload = build_transcript_payload(&transcript_embedding).map_err(|e| {
            error!("Error building transcript payload: {}", e);
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
            .map_err(|e| {
                error!("Failed to upsert points: {}", e);
                Status::internal(format!("Failed to upsert points: {}", e))
            })?;

        debug!("Successfully upserted embeddings with payload");

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

        // let payload = build_summary_payload(&summary_embedding);

        // Upsert embedding
        println!("{}", summary_embedding.lecture_name);
        Err(Status::unimplemented("Not implemented yet"))
    }

    async fn similarity_search(
        &self,
        request: Request<SimilaritySearchReq>,
    ) -> std::result::Result<Response<SimilaritySearchRes>, tonic::Status> {
        let req = request.into_inner();
        let vector = req.embedding.ok_or_else(|| {
            Status::invalid_argument("Field embedding is missing in request for similarity search")
        })?;

        // TODO similarity search should also be possible for _summary collections
        let res = self
            .client
            .query(
                QueryPointsBuilder::new(format!("{}_embedding", req.module).clone())
                    .query(Query::new_nearest(vector.vector_data))
                    .with_payload(true)
                    .with_vectors(true)
                    .limit(5),
            )
            .await
            .map_err(|e| {
                error!("Similarity search failed: {}", e);
                Status::internal(format!("Similarity search failed: {}", e))
            })?;

        let mut result_docs: Vec<TranscriptEmbedding> = Vec::new();

        for point in res.result {
            let timestamp_start = extract_f32_from_payload(&point.payload, "timestamp_start")?;
            let timestamp_end = extract_f32_from_payload(&point.payload, "timestamp_end")?;
            let raw_content = extract_string_from_payload(&point.payload, "raw_content")?;
            let lecture_name = extract_string_from_payload(&point.payload, "lecture_name")?;

            result_docs.push(TranscriptEmbedding {
                module: req.module.clone(),
                timestamp: Some(Timestamp {
                    timestamp_start,
                    timestamp_end,
                }),
                raw_content,
                lecture_name,
                embedding: None,
            });
        }

        Ok(Response::new(SimilaritySearchRes { result_docs }))
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
                .map_err(|e| {
                    error!("Failed to create collection '{}': {}", collection_name, e);
                    Status::internal(format!("Failed to create collection: {}", e))
                })?;
            debug!("Collection {} created", collection_name);
            Ok(())
        }
        Err(e) => {
            error!(
                "Failed to check if collection '{}' exists: {}",
                collection_name, e
            );
            Err(Status::internal(format!(
                "Failed to check if collection exists: {}",
                e
            )))
        }
        _ => {
            debug!("Collection {} exists", collection_name);
            Ok(())
        } // Do nothing as collection already exists
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

    debug!(
        "Built transcript payload with timestamp_start: {}, timestamp_end: {}, raw_content: {}, lecture_name: {}",
        timestamp.timestamp_start,
        timestamp.timestamp_end,
        transcript_embedding.raw_content,
        transcript_embedding.lecture_name
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

    debug!(
        "Built summary payload with raw_content: {}, lecture_name:{}",
        summary_embedding.raw_content, summary_embedding.lecture_name
    );

    payload
}

fn extract_f32_from_payload(
    payload: &HashMap<String, Value>,
    field_name: &str,
) -> Result<f32, Status> {
    debug!("{:?}", payload);
    payload
        .get(field_name)
        .and_then(|v| match v.kind.as_ref()? {
            qdrant_client::qdrant::value::Kind::DoubleValue(d) => Some(*d as f32),
            qdrant_client::qdrant::value::Kind::IntegerValue(i) => Some(*i as f32),
            _ => None,
        })
        .ok_or_else(|| {
            Status::internal(format!(
                "Point was missing field {} or it did not include a numeric value",
                field_name
            ))
        })
}

fn extract_string_from_payload(
    payload: &HashMap<String, Value>,
    field_name: &str,
) -> Result<String, Status> {
    payload
        .get(field_name)
        .and_then(|v| match v.kind.as_ref()? {
            qdrant_client::qdrant::value::Kind::StringValue(s) => Some(s.clone()),
            _ => None,
        })
        .ok_or_else(|| {
            Status::internal(format!(
                "Point was missing field {} or it was not a string",
                field_name
            ))
        })
}
