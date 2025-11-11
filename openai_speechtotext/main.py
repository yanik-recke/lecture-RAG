import json
import os
from dotenv import load_dotenv
from openai import OpenAI
import time


# This script uses the OpenAI API to transcribe lectures.
# You have to modify the strings in the path list
# to contain the files you want to convert. Transcription
# for two files usually takes up to 10 to 20 minutes.
# The results are appended to a specified file
# for later use (for example embedding the segments).
#
# API reference:
# https://platform.openai.com/docs/api-reference/audio/createTranscription
def main():
    load_dotenv(".env")
    client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    paths = [
        "/Users/yanik/_repos/lecture-RAG/openai_speechtotext/files/bk1a_part1.mp3",
        "/Users/yanik/_repos/lecture-RAG/openai_speechtotext/files/bk1a_part2.mp3"
    ]

    results = []

    for idx, path in enumerate(paths):
        audio_file = open(path, "rb")

        # Set timestamp_granularities to "segment" to already
        # split up the text in parts that can be embedded.
        # You may also set it to "words", but that makes embedding
        # the content harder.
        # https://platform.openai.com/docs/api-reference/audio/createTranscription#audio_createtranscription-timestamp_granularities
        transcription = client.audio.transcriptions.create(
            file=audio_file,
            model="whisper-1",
            response_format="verbose_json",
            timestamp_granularities=["segment"]
        )

        transcription_dict = transcription.model_dump()

        # Adjust timestamps for split files (each part is 2250 seconds)
        if idx > 0:
            time_offset = idx * 2250

            # Adjust segment timestamps
            if transcription_dict.get("segments"):
                for segment in transcription_dict["segments"]:
                    segment["start"] += time_offset
                    segment["end"] += time_offset

            # Adjust word timestamps if present
            if transcription_dict.get("words"):
                for word in transcription_dict["words"]:
                    word["start"] += time_offset
                    word["end"] += time_offset

        results.append({
            "file": path,
            "transcription": transcription_dict
        })

    with open("transcription-BK1a.json", "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)


if __name__ == "__main__":
    start_time = time.time()
    main()
    print("--- %s seconds ---" % (time.time() - start_time))
