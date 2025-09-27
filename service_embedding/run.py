from sentence_transformers import SentenceTransformer
from dotenv import load_dotenv
from flask import Flask, request, jsonify
from pymilvus import MilvusClient
import os
import torch

try: 
    load_dotenv(".env")
except:
    print("No .env file found.")

hf_token = os.environ.get("HF_TOKEN")

if (hf_token == None):
    print("Please set a valid hugging face token.")
    raise Exception("Missing HF Token")


db_url = os.environ.get("DB_URL")

if (db_url == None):
    print("Please set a valid database connection URL.")
    raise Exception("Missing DB Url")

# Needed due to an issue with torch 
# https://stackoverflow.com/questions/72641886/attributeerror-module-torch-distributed-has-no-attribute-is-initialized-in
setattr(torch.distributed, "is_initialized", lambda : False)

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model = SentenceTransformer("google/embeddinggemma-300m", token=hf_token)
model.to(device)

# DB client initialization
client = MilvusClient(db_url, timeout=10)

app = Flask(__name__)

@app.route('/health', methods=['GET'])
def health():
    return jsonify({'status': 'healthy'})

@app.route('/embed', methods=['POST'])
def embed_text():
    data = request.get_json()

    text = data.get('text', '')

    if not text:
        return jsonify({'error': 'No text provided'}), 400
    
    module = data.get('module', '')

    if not module:
        return jsonify({'error': 'No module provided'}), 400
    
    url = data.get('url', '')

    if not url:
        return jsonify({'error': 'No URL provided'}), 400

    embeddings = model.encode_query(text)

    if not client.has_collection(module):
        return jsonify({'error': 'No collection with that name exists. Create one first.'})

    res = client.insert(
        collection_name=module,
        data={
            'vector': embeddings.tolist(),
            'lectureUrl': url
        }
    )

    if res.get('insert_count') < 1:
        return jsonify({'error': 'Embeddings created, but not inserted into vector store'}), 400

    return jsonify({'embeddings': embeddings.tolist()})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8001, debug=True)
