use crate::metadataservice::metadata_service_server::MetadataService;
use crate::metadataservice::{
    DeleteLectureReq, GetModulesNamesReq, GetModulesNamesRes, GetModulesReq, GetModulesRes,
    GetSummaryReq, GetSummaryRes,
};
use crate::repository::MetadataServicer;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl MetadataService for MetadataServicer {
    /// Gets the names of all the modules saved in the database.
    async fn get_modules_names(
        &self,
        _request: Request<GetModulesNamesReq>,
    ) -> anyhow::Result<Response<GetModulesNamesRes>, Status> {
        let names = self
            .fetch_module_names()
            .await
            .map_err(|e| Status::internal(format!("Could not retrieve module names: {}", e)))?;

        Ok(Response::new(GetModulesNamesRes { names }))
    }

    /// Get all modules, including the lectures
    async fn get_modules(
        &self,
        _request: Request<GetModulesReq>,
    ) -> anyhow::Result<Response<GetModulesRes>, Status> {
        let modules = self
            .fetch_modules()
            .await
            .map_err(|e| Status::internal(format!("Could not retrieve modules: {}", e)))?;

        Ok(Response::new(GetModulesRes { modules }))
    }

    /// Get the specific summary of a lecture
    async fn get_summary(
        &self,
        _request: Request<GetSummaryReq>,
    ) -> anyhow::Result<Response<GetSummaryRes>, Status> {
        todo!()
    }

    /// Delete a lecture by its ID
    async fn delete_lecture(
        &self,
        request: Request<DeleteLectureReq>,
    ) -> anyhow::Result<Response<()>, Status> {
        self.delete_lecture_by_id(request.into_inner().lecture_id)
            .await
            .map_err(|e| Status::internal(format!("Could not delete lecture {}", e)))?;

        Ok(Response::new(()))
    }
}
