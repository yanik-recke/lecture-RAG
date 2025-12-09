use anyhow::{Context, Error};
use mongodb::{Client, Collection};
use service_metadata::models::{MongoMetadataLecture, MongoMetadataModule};
use service_metadata::repository::MetadataServicer;

async fn init() -> Result<MetadataServicer, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let database = client.database(&*"test_db");

    let module_coll: Collection<MongoMetadataModule> = database.collection(&*"test_modules");
    let lecture_coll: Collection<MongoMetadataLecture> = database.collection(&*"test_lectures");

    Ok(MetadataServicer::new(module_coll, lecture_coll))
}

#[tokio::test]
async fn test_fetch_module_names() {
    env_logger::builder().is_test(true).try_init().ok();
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
    for name in names {
        println!("Module: {}", name);
    }
}
