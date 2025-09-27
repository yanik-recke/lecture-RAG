import torch
import time
import os
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor, pipeline
from flask import Flask, request, jsonify

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
    
    file = request.files['audio']

    if file:
        # Save file
        filename = str(int(time.time())) + file.filename
        file.save(filename)

        result = pipe(filename, generate_kwargs={'language': 'german'})

        # Remove file
        os.remove(filename)
        
        return jsonify({'result': result['text']})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8003, debug=True)
