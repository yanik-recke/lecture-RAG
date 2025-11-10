import json
import os
from dotenv import load_dotenv
from openai import OpenAI

def main():
    load_dotenv(".env")
    client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    paths = [
        "/Users/yanik/_repos/lecture-RAG/openai_speechtotext/files/out1.mp3",
        "/Users/yanik/_repos/lecture-RAG/openai_speechtotext/files/out2.mp3"
    ]

    results = []

    for path in paths:
        audio_file = open(path, "rb")

        transcription = client.audio.transcriptions.create(
            file=audio_file,
            model="whisper-1",
            response_format="verbose_json",
            timestamp_granularities=["word"]
        )

        results.append({
            "file": path,
            "transcription": transcription.model_dump()
        })

    with open("transcriptions.json", "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)


if __name__ == "__main__":
    main()
