import grpc
from sentence_transformers import SentenceTransformer
from dotenv import load_dotenv
import os
# Torch is provided by the Nvidia Docker container
import torch
import target.lecture_store_pb2 as lecture_store_pb2
import target.lecture_store_pb2_grpc as lecture_store_pb2_grpc

# ===================================
# This service is supposed to be run
# on the Nvidia Jetson Orin Nano.
# ===================================

try: 
    load_dotenv(".env")
except:
    print("No .env file found.")

hf_token = os.environ.get("HF_TOKEN")

if hf_token is None:
    print("Please set a valid hugging face token.")
    raise Exception("Missing HF Token")


db_url = os.environ.get("DB_URL")

if db_url is None:
    print("Please set a valid database connection URL.")
    raise Exception("Missing DB Url")

# Needed due to an issue with torch 
# https://stackoverflow.com/questions/72641886/attributeerror-module-torch-distributed-has-no-attribute-is-initialized-in
setattr(torch.distributed, "is_initialized", lambda : False)

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model = SentenceTransformer("google/embeddinggemma-300m", token=hf_token)
model.to(device)

# Instantiate channel and stub
channel = grpc.insecure_channel("localhost:19000")
stub = lecture_store_pb2_grpc.LectureStoreStub(channel)
