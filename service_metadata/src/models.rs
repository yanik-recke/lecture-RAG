use crate::metadataservice::MetadataLecture;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoMetadataModule {
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoMetadataLecture {
    pub(crate) name: String,
    pub(crate) lecture_id: String,
    pub(crate) summary: String,
    pub(crate) module_name: String,
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
