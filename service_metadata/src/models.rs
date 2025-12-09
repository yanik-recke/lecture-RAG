use crate::metadataservice::{MetadataLecture, MetadataModule};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoMetadataModule {
    pub(crate) name: String,
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
