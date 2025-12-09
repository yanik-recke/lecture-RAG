mod models;
mod repository;
mod service;

use crate::metadataservice::metadata_service_server::MetadataServiceServer;
use crate::models::{MongoMetadataLecture, MongoMetadataModule};
use crate::repository::MetadataServicer;
use anyhow::{Context, Result};
use log::info;
use mongodb::{Client, Collection};
use std::net::SocketAddr;
use tonic::transport::Server;

pub mod metadataservice {
    tonic::include_proto!("metadataservice");
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let host =
        std::env::var("METADATA_SERVICE_HOST").context("METADATA_SERVICE_HOST must be set")?;
    let port =
        std::env::var("METADATA_SERVICE_PORT").context("METADATA_SERVICE_PORT must be set")?;

    let db_uri = std::env::var("METADATA_DB_URI").context("METADATA_DB_URI must be set")?;
    let db_name = std::env::var("METADATA_DB_NAME").context("METADATA_DB_NAME must be set")?;
    let db_coll = std::env::var("METADATA_DB_COLL").context("METADATA_DB_COLL must be set")?;

    let client = Client::with_uri_str(db_uri).await?;

    let database = client.database(&*db_name);

    let module_coll: Collection<MongoMetadataModule> = database.collection(&*db_coll);

    let lecture_coll: Collection<MongoMetadataLecture> = database.collection(&*db_coll);

    let metadata_servicer = MetadataServicer::new(module_coll, lecture_coll);

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
