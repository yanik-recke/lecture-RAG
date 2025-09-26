from pymilvus import MilvusClient, DataType

# ========================
# Initialization script for the milvus vector store.
# Schema definition, etc.
#
# Make sure to include the correct destination in the client creation.
# ========================

client = MilvusClient("http://localhost:19530", timeout=10_000)

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
    dim=256,
)

client.create_collection(
    collection_name="snippets",
    schema=schema
)

if (client.has_collection(collection_name="snippets")):
    print("Yes")
else:
    print("No")