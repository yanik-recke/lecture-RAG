from pymilvus import MilvusClient, DataType
import os

# ========================
# Initialization script for the milvus vector store.
# Schema definition, etc.
#
# Make sure to include the correct destination in the client creation.
# ========================
db_url = os.environ.get("DB_URL")

if (db_url == None):
    # Default Jetson Orin Nano adress, when using wired connection
    db_url = "http://192.168.55.1:19530"

client = MilvusClient(db_url, timeout=10)

if (client.has_collection(collection_name="snippets")):
    print("Already exists, exiting.")
else:
    print("Creating collection.")

    schema = MilvusClient.create_schema();

    # Autogenerate ID
    schema.add_field(
        field_name="id",
        datatype=DataType.INT64,
        is_primary=True,
        auto_id=True,
    )

    # Vector field
    schema.add_field(
        field_name="vector",
        datatype=DataType.FLOAT_VECTOR,
        dim=768,
    )

    # Field with URL to lecture video
    schema.add_field(
        field_name="lecture_url",
        datatype=DataType.VARCHAR,
        max_length=512,
        nullable=False
    )

    client.create_collection(
        collection_name="snippets",
        schema=schema
    )