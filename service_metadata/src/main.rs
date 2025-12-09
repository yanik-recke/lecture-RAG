use crate::metadataservice::metadata_service_server::{MetadataService, MetadataServiceServer};
use crate::metadataservice::{
    DeleteLectureReq, GetModulesNamesReq, GetModulesNamesRes, GetModulesReq, GetModulesRes,
    GetSummaryReq, GetSummaryRes,
};
use anyhow::{Context, Result};
use log::info;
use mongodb::bson::Document;
use mongodb::{Client, Collection};
use std::fs::metadata;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod metadataservice {
    tonic::include_proto!("metadataservice");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let host =
        std::env::var("METADATA_SERVICE_HOST").context("METADATA_SERVICE_HOST must be set")?;
    let port =
        std::env::var("METADATA_SERVICE_PORT").context("METADATA_SERVICE_PORT must be set")?;

    let db_uri = std::env::var("METADATA_DB_URI").context("METADATA_DB_URI must be set")?;
    let db_name = std::env::var("METADATA_DB_NAME").context("METADATA_DB_NAME must be set")?;
    let db_coll = std::env::var("METADATA_DB_COLL").context("METADATA_DB_COLL must be set")?;

    let client = Client::with_uri_str(db_uri).await?;

    let database = client.database(&*db_name);

    let metadata_coll: Collection<Document> = database.collection(&*db_coll);

    let metadata_servicer = MetadataServicer::new(metadata_coll);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .context("Could not create socket address")?;

    info!("Starting server on {}:{}", host, port);

    Server::builder()
        .add_service(MetadataServiceServer::new(metadata_servicer))
        .serve(addr)
        .await
        .context("Could not build server")?;

    Ok(())
}

struct MetadataServicer {
    metadata_coll: mongodb::Collection<Document>,
}

impl MetadataServicer {
    pub fn new(metadata_coll: Collection<Document>) -> Self {
        MetadataServicer { metadata_coll }
    }
}

#[tonic::async_trait]
impl MetadataService for MetadataServicer {
    async fn get_modules_names(
        &self,
        request: Request<GetModulesNamesReq>,
    ) -> Result<Response<GetModulesNamesRes>, Status> {
        todo!()
    }

    async fn get_modules(
        &self,
        request: Request<GetModulesReq>,
    ) -> Result<Response<GetModulesRes>, Status> {
        todo!()
    }

    async fn get_summary(
        &self,
        request: Request<GetSummaryReq>,
    ) -> Result<Response<GetSummaryRes>, Status> {
        todo!()
    }

    async fn delete_lecture(
        &self,
        request: Request<DeleteLectureReq>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }
}
