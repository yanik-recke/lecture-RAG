use crate::lectureservice::TranscribeReq;
use crate::whisperservice_mod::whisper_service_client::WhisperServiceClient;
use anyhow::{Context, Result};
use tonic::transport::{Channel, Endpoint};

struct WhisperService {
    client: WhisperServiceClient<Channel>,
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
            client: WhisperServiceClient::new(channel),
        })
    }

    pub async fn transcribe(&mut self, trans_req: TranscribeReq) -> Result<()> {
        self.client
            .transcribe(trans_req)
            .await
            .context("Request to transcription service failed")?;
        Ok(())
    }
}
