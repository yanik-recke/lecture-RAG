import target.embedding_service_pb2_grpc as embedding_grpc
import target.embedding_service_pb2 as embedding_pb2
import target.lecture_store_pb2 as lecture_store_pb2

from concurrent import futures
import grpc

from dotenv import load_dotenv
import os
import torch
from sentence_transformers import SentenceTransformer

try:
    load_dotenv(".env")
except:
    print("No .env file found.")

hf_token = os.environ.get("HF_TOKEN")

if hf_token is None:
    print("Please set a valid hugging face token.")
    raise Exception("Missing HF Token")

# Needed due to an issue with torch
# https://stackoverflow.com/questions/72641886/attributeerror-module-torch-distributed-has-no-attribute-is-initialized-in
setattr(torch.distributed, "is_initialized", lambda: False)


class EmbeddingServiceServicer(embedding_grpc.EmbeddingServiceServicer):

    def __init__(self):
        device = "cuda:0" if torch.cuda.is_available() else "cpu"

        self.model = SentenceTransformer("google/embeddinggemma-300m", token=hf_token)
        self.model.to(device)

    def CreateEmbedding(self, request, context):
        query = request.to_embed.raw_content

        # Convert embeddings to a list of floats
        embedding_list = self.model.encode_query(query).tolist()

        # Create the VectorEmbedding message
        vector_embedding = lecture_store_pb2.VectorEmbedding(vector_data=embedding_list)

        # Return the response with the vector embedding
        return embedding_pb2.NewEmbeddingRes(vec=vector_embedding)


def serve():
    """Start the gRPC server."""
    port = os.environ.get("WHISPER_PORT", "50051")
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))

    embedding_grpc.add_EmbeddingServiceServicer_to_server(EmbeddingServiceServicer(), server)

    server.add_insecure_port(f'[::]:{port}')
    server.start()

    print(f"Embedding gRPC server started on port {port}")

    try:
        server.wait_for_termination()
    except KeyboardInterrupt:
        print("\nShutting down server...")
        server.stop(0)


if __name__ == '__main__':
    serve()
