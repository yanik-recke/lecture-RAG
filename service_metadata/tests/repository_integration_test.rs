use anyhow::{Context, Error};
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use service_metadata::models::{MongoMetadataLecture, MongoMetadataModule};
use service_metadata::repository::MetadataServicer;

async fn init() -> Result<MetadataServicer, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let database = client.database(&*"lecture_metadata");

    let module_coll: Collection<MongoMetadataModule> = database.collection(&*"modules");
    let lecture_coll: Collection<MongoMetadataLecture> = database.collection(&*"lectures");

    // Clear the collections
    module_coll.delete_many(doc! {}).await?;
    lecture_coll.delete_many(doc! {}).await?;

    Ok(MetadataServicer::new(module_coll, lecture_coll))
}

#[tokio::test]
async fn test_adding_module_duplicate_name() {
    let servicer = init().await.context("Failed to create servicer").unwrap();
    servicer
        .add_new_module("Mod 66".to_string())
        .await
        .context("Failed to add module")
        .unwrap();

    servicer
        .add_new_module("Mod 66".to_string())
        .await
        .unwrap_err();
}

#[tokio::test]
async fn test_fetch_module_names() {
    let servicer = init().await.context("Failed to create servicer").unwrap();

    let initial_module_len = servicer
        .fetch_module_names()
        .await
        .map_err(|e| format!("Failed to fetch module names {}", e))
        .unwrap()
        .len();

    for name in vec!["Mod 1", "Mod 2", "Mod 3"] {
        servicer
            .add_new_module(name.to_string())
            .await
            .context(format! {"Failed to add module {}", name})
            .unwrap();
    }

    let names = servicer
        .fetch_module_names()
        .await
        .map_err(|e| format!("Failed to fetch module names {}", e))
        .unwrap();

    assert_eq!(initial_module_len + 3, names.len());
}
