use crate::lectureservice::TranscribeReq;
use crate::whisperservice::TranscribedRes;
use crate::whisperservice::whisper_service_client::WhisperServiceClient;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};

pub struct WhisperService {
    client: Arc<Mutex<WhisperServiceClient<Channel>>>,
}

impl WhisperService {
    pub async fn new(host: String, port: u32) -> Result<Self> {
        let endpoint =
            Endpoint::new(format!("{}:{}", host, port)).context("Could not create endpoint")?;
        let channel = endpoint
            .connect()
            .await
            .context("Failed to connect to server")?;

        Ok(WhisperService {
            client: Arc::new(Mutex::new(WhisperServiceClient::new(channel))),
        })
    }

    pub async fn transcribe(&self, trans_req: TranscribeReq) -> Result<TranscribedRes> {
        Ok(self
            .client
            .lock()
            .await
            .transcribe(trans_req)
            .await
            .context("Request to transcription service failed")?
            .into_inner())
    }
}
