use crate::metadataservice::metadata_service_server::{MetadataService, MetadataServiceServer};
use crate::metadataservice::{
    DeleteLectureReq, GetModulesNamesReq, GetModulesNamesRes, GetModulesReq, GetModulesRes,
    GetSummaryReq, GetSummaryRes, MetadataLecture, MetadataModule,
};
use anyhow::{Context, Result};
use log::info;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

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

struct MetadataServicer {
    module_coll: Collection<MongoMetadataModule>,
    lecture_coll: Collection<MongoMetadataLecture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoMetadataModule {
    name: String,
    lectures: Vec<MongoMetadataLecture>,
}

impl From<MongoMetadataModule> for MetadataModule {
    fn from(value: MongoMetadataModule) -> Self {
        MetadataModule {
            name: value.name,
            lectures: value.lectures.into_iter().map(|l| l.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoMetadataLecture {
    name: String,
    lecture_id: String,
    summary: String,
}

impl From<MongoMetadataLecture> for MetadataLecture {
    fn from(value: MongoMetadataLecture) -> Self {
        MetadataLecture {
            name: value.name,
            lecture_id: value.lecture_id,
            summary: value.summary,
        }
    }
}

impl MetadataServicer {
    pub fn new(
        module_coll: Collection<MongoMetadataModule>,
        lecture_coll: Collection<MongoMetadataLecture>,
    ) -> Self {
        MetadataServicer {
            module_coll,
            lecture_coll,
        }
    }

    async fn fetch_module_names(&self) -> Result<Vec<String>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut names = Vec::new();

        while let Some(result) = cursor.next().await {
            names.push(result?.name);
        }

        Ok(names)
    }

    async fn fetch_modules(&self) -> Result<Vec<MetadataModule>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut modules = Vec::new();

        while let Some(result) = cursor.next().await {
            modules.push(MetadataModule::from(result?))
        }

        Ok(modules)
    }
}

#[tonic::async_trait]
impl MetadataService for MetadataServicer {
    /// Gets the names of all the modules saved in the database.
    async fn get_modules_names(
        &self,
        _: Request<GetModulesNamesReq>,
    ) -> Result<Response<GetModulesNamesRes>, Status> {
        let names = self
            .fetch_module_names()
            .await
            .map_err(|e| Status::internal(format!("Could not retrieve module names: {}", e)))?;

        Ok(Response::new(GetModulesNamesRes { names }))
    }

    /// Get all modules, including the lectures
    async fn get_modules(
        &self,
        _: Request<GetModulesReq>,
    ) -> Result<Response<GetModulesRes>, Status> {
        let modules = self
            .fetch_modules()
            .await
            .map_err(|e| Status::internal(format!("Could not retrieve modules: {}", e)))?;

        Ok(Response::new(GetModulesRes { modules }))
    }

    /// Get the specific summary of a lecture
    async fn get_summary(
        &self,
        request: Request<GetSummaryReq>,
    ) -> Result<Response<GetSummaryRes>, Status> {
        todo!()
    }

    /// Delete a lecture by its ID
    async fn delete_lecture(
        &self,
        request: Request<DeleteLectureReq>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }
}
