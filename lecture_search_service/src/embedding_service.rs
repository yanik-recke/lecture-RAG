use crate::embeddingservice::embedding_service_client::EmbeddingServiceClient;
use crate::embeddingservice::{NewEmbeddingReq, NewEmbeddingRes, RawText};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Request;
use tonic::transport::{Channel, Endpoint};

pub struct EmbeddingService {
    client: Arc<Mutex<EmbeddingServiceClient<Channel>>>,
}

impl EmbeddingService {
    pub async fn new(host: String, port: u32) -> Result<Self> {
        let endpoint =
            Endpoint::new(format!("{}:{}", host, port)).context("Could not create endpoint")?;
        let channel = endpoint
            .connect()
            .await
            .context("Failed to connect to server")?;

        Ok(EmbeddingService {
            client: Arc::new(Mutex::new(EmbeddingServiceClient::new(channel))),
        })
    }

    pub async fn embed(&self, text: String) -> Result<NewEmbeddingRes> {
        let req = Request::new(NewEmbeddingReq {
            to_embed: Some(RawText { raw_content: text }),
        });

        Ok(self
            .client
            .lock()
            .await
            .create_embedding(req)
            .await
            .context("Failed to create embedding")?
            .into_inner())
    }
}
