import torch
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor, pipeline, GenerationConfig
import gradio as gr
import time
import json

device = "cuda:0" if torch.cuda.is_available() else "cpu"
torch_dtype = torch.float16 if torch.cuda.is_available() else torch.float32

# model_id = "primeline/whisper-large-v3-german"
model_id = "primeline/whisper-large-v3-turbo-german"
# model_id = "primeline/whisper-tiny-german"

model = AutoModelForSpeechSeq2Seq.from_pretrained(
    model_id, torch_dtype=torch_dtype, low_cpu_mem_usage=True, use_safetensors=True
)

# generation_config = GenerationConfig.from_pretrained("openai/whisper-base")
# model.generation_config = generation_config
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


def transcribe_audio(path):
    # , return_timestamps="word"
    start_time = time.time()
    print(path)

    result = pipe(path, generate_kwargs={"language": "german"})

    print(result['text'])
    print("------")
    print(result['chunks'])
    print("--- %s seconds ---" % (time.time() - start_time))

    with open("/app/transcriptions/transcription.json", "w", encoding="utf-8") as f:
        json.dump(result, f, indent=2, ensure_ascii=False)

    return result["text"]


if __name__ == "__main__":
    interface = gr.Interface(
        fn=transcribe_audio,
        inputs=gr.Audio(type="filepath", label="Upload Audio (German)"),
        outputs="text",
        title="German Whisper ASR",
        description="Upload an audio file to get its German transcription."
    )
    interface.launch(server_name="0.0.0.0", share=False)
