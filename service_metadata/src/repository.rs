use crate::metadataservice::{MetadataLecture, MetadataModule};
use crate::models::{MongoMetadataLecture, MongoMetadataModule};
use futures::StreamExt;
use log::{debug, warn};
use mongodb::Collection;
use mongodb::bson::doc;
use uuid::Uuid;

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

    pub async fn add_new_module(&self, name: String) -> anyhow::Result<(), mongodb::error::Error> {
        let module = MongoMetadataModule { name };

        self.module_coll.insert_one(module).await?;

        Ok(())
    }

    pub async fn add_new_lecture(
        &self,
        name: String,
        module_name: String,
        summary: String,
    ) -> anyhow::Result<(), mongodb::error::Error> {
        // If no collection with that name exists, create one
        if self
            .module_coll
            .find_one(doc! {"name": name.clone()})
            .await?
            .is_none()
        {
            self.add_new_module(name.clone()).await?;
        }

        // Create lecture object and insert into collection
        let lecture = MongoMetadataLecture {
            name,
            summary,
            module_name,
            lecture_id: Uuid::new_v4().to_string(),
        };

        self.lecture_coll.insert_one(lecture).await?;

        Ok(())
    }

    pub async fn fetch_module_names(&self) -> anyhow::Result<Vec<String>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut names = Vec::new();

        while let Some(result) = cursor.next().await {
            let module = result?;
            debug!("Adding module name: {}", module.name);
            names.push(module.name);
        }

        Ok(names)
    }

    pub async fn fetch_modules(
        &self,
    ) -> anyhow::Result<Vec<MetadataModule>, mongodb::error::Error> {
        let mut cursor = self.module_coll.find(doc! {}).await?;
        let mut modules = Vec::new();

        while let Some(result) = cursor.next().await {
            let module_name = result?.name;

            let mut lecture_cursor = self
                .lecture_coll
                .find(doc! {"module_id": module_name.clone()})
                .await?;

            let mut lectures = Vec::new();

            while let Some(lecture) = lecture_cursor.next().await {
                let lec = lecture?;
                lectures.push(MetadataLecture {
                    name: lec.module_name,
                    summary: lec.summary,
                    lecture_id: lec.lecture_id,
                })
            }

            modules.push(MetadataModule {
                name: module_name,
                lectures,
            });
        }

        Ok(modules)
    }

    pub async fn delete_lecture_by_id(
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
