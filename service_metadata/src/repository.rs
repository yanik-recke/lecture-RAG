use crate::metadataservice::MetadataModule;
use crate::models::{MongoMetadataLecture, MongoMetadataModule};
use futures::StreamExt;
use log::warn;
use mongodb::Collection;
use mongodb::bson::doc;

pub struct MetadataServicer {
    module_coll: Collection<MongoMetadataModule>,
    lecture_coll: Collection<MongoMetadataLecture>,
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

    pub(crate) async fn fetch_module_names(
        &self,
    ) -> anyhow::Result<Vec<String>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut names = Vec::new();

        while let Some(result) = cursor.next().await {
            names.push(result?.name);
        }

        Ok(names)
    }

    pub(crate) async fn fetch_modules(
        &self,
    ) -> anyhow::Result<Vec<MetadataModule>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut modules = Vec::new();

        while let Some(result) = cursor.next().await {
            modules.push(MetadataModule::from(result?))
        }

        Ok(modules)
    }

    pub(crate) async fn delete_lecture_by_id(
        &self,
        lecture_id: String,
    ) -> anyhow::Result<(), mongodb::error::Error> {
        let res = self
            .lecture_coll
            .delete_one(doc! {"lecture_id": lecture_id.clone()})
            .await?;

        if res.deleted_count != 1 {
            warn!("Did not delete lecture with Id {}", lecture_id);
        }

        Ok(())
    }
}
