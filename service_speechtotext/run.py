import torch
import time
import os
import sys
import requests
from dotenv import load_dotenv
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor, pipeline
from flask import Flask, request, jsonify

try: 
    load_dotenv(".env")
except:
    print("No .env file found.")

embedding_url = os.environ.get("EMBEDDING-URL")

if embedding_url == None:
    embedding_url = "http://192.168.55.1:8002/"

device = "cuda:0" if torch.cuda.is_available() else "cpu"
torch_dtype = torch.float16 if torch.cuda.is_available() else torch.float32
model_id = "primeline/whisper-large-v3-turbo-german"

model = AutoModelForSpeechSeq2Seq.from_pretrained(
    model_id, dtype=torch_dtype, low_cpu_mem_usage=True, use_safetensors=True
)

model.to(device)

processor = AutoProcessor.from_pretrained(model_id)
pipe = pipeline(
    "automatic-speech-recognition",
    model=model,
    processor=processor,
    tokenizer=processor.tokenizer,
    feature_extractor=processor.feature_extractor,
    device=device,
    torch_dtype=torch_dtype,
    return_timestamps=True
)

# Routes
app = Flask(__name__)

@app.route('/health', methods=['GET'])
def health():
    return jsonify({'status':'healthy'})

@app.route('/transcribe', methods=['POST'])
def transcribe():
    if 'audio' not in request.files:
        return jsonify({'error': 'No audio file provided'}), 400
    
    module = request.form.get('module')
    if module == None:
        return jsonify({'error': 'No module provided'}), 400

    url = request.form.get('url')
    if url == None:
        return jsonify({'error': 'No url provided'}), 400

    file = request.files['audio']

    if file:
        result = None
        # Save file
        filename = str(int(time.time())) + file.filename
        try:    
            file.save(filename)

            result = pipe(filename, generate_kwargs={'language': 'german'})
            
            for chunk in result['chunks']:
                res = requests.post(embedding_url + '/embed', json={
                    'text': chunk['text'],
                    'module': module,
                    'url': url,
                    'timestamp_start': 0.1 if chunk['timestamp'][0] == 0 else chunk['timestamp'][0],
                    'timestamp_end': chunk['timestamp'][1]
                })

                if not res.ok:
                    print('Error while sending chunk to embed endpoint:' + res.json())

            return jsonify(result)
        except Exception as e:
            print("Error: ", e, file=sys.stderr)
            return jsonify({'error': 'Error while trying to transcribe file.'})
        finally:
            # Remove the file
            os.remove(filename)

if __name__ == '__main__':
    # Having processes set to anyhing other than 1 causes OOM
    app.run(host='0.0.0.0', port=8003, processes=1)
