from sentence_transformers import SentenceTransformer
from dotenv import load_dotenv
from flask import Flask, request, jsonify
import os
import torch

load_dotenv(".env")
hf_token = os.environ.get("HF_TOKEN")

# Needed due to an issue with torch 
# https://stackoverflow.com/questions/72641886/attributeerror-module-torch-distributed-has-no-attribute-is-initialized-in
setattr(torch.distributed, "is_initialized", lambda : False)

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model = SentenceTransformer("google/embeddinggemma-300m", token=hf_token)
model.to(device)


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

    embeddings = model.encode_query(text)
    return jsonify({'embeddings': embeddings.tolist()})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8001, debug=True)
