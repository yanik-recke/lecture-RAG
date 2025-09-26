from sentence_transformers import SentenceTransformer
from dotenv import load_dotenv
import os
import torch

load_dotenv(".env")
hf_token = os.environ.get("HF_TOKEN")

# Needed due to an issue with torch 
# https://stackoverflow.com/questions/72641886/attributeerror-module-torch-distributed-has-no-attribute-is-initialized-in
setattr(torch.distributed, "is_initialized", lambda : False)

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model = SentenceTransformer("google/embeddinggemma-300m", token="")

model.to(device)


while True:
    query = input()
    documents = [
        "Venus is often called Earth's twin because of its similar size and proximity.",
        "Mars, known for its reddish appearance, is often referred to as the Red Planet.",
        "Jupiter, the largest planet in our solar system, has a prominent red spot.",
        "Saturn, famous for its rings, is sometimes mistaken for the Red Planet."
    ]

    query_embeddings = model.encode_query(query)
    document_embeddings = model.encode_document(documents)
    print(query_embeddings.shape, document_embeddings.shape)
