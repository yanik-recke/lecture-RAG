use crate::lectureservice::TranscribeReq;
use crate::whisperservice::TranscribedRes;
use crate::whisperservice::whisper_service_client::WhisperServiceClient;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Status;
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

        let client = WhisperServiceClient::new(channel)
            .max_decoding_message_size(100 * 1024 * 1024) // 100 MB
            .max_encoding_message_size(100 * 1024 * 1024); // 100 MB

        Ok(WhisperService {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn transcribe(&self, trans_req: TranscribeReq) -> Result<TranscribedRes> {
        Ok(self
            .client
            .lock()
            .await
            .transcribe(trans_req)
            .await
            .map_err(|e| {
                log::error!(
                    "Transcription request failed - Status: {:?}, Message: {}, Details: {:?}",
                    e.code(),
                    e.message(),
                    e.metadata()
                );
                Status::internal(format!(
                    "Transcription failed: {} (status: {:?})",
                    e.message(),
                    e.code()
                ))
            })?
            .into_inner())
    }
}
