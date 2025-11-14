import grpc
from concurrent import futures
import torch
import os
import sys
import tempfile
from dotenv import load_dotenv
from transformers import AutoModelForSpeechSeq2Seq, AutoProcessor, pipeline

import target.whisper_service_pb2 as whisper_pb2
import target.whisper_service_pb2_grpc as whisper_grpc
import target.lecture_store_pb2 as lecture_store_pb2

try:
    load_dotenv(".env")
except:
    print("No .env file found.")


class WhisperServiceServicer(whisper_grpc.WhisperServiceServicer):
    def __init__(self):
        device = "cuda:0" if torch.cuda.is_available() else "cpu"
        torch_dtype = torch.float16 if torch.cuda.is_available() else torch.float32

        model_id = "primeline/whisper-large-v3-turbo-german"

        model = AutoModelForSpeechSeq2Seq.from_pretrained(
            model_id, torch_dtype=torch_dtype, low_cpu_mem_usage=True, use_safetensors=True
        )

        model.to(device)

        processor = AutoProcessor.from_pretrained(model_id)
        self.pipe = pipeline(
            "automatic-speech-recognition",
            model=model,
            processor=processor,
            tokenizer=processor.tokenizer,
            feature_extractor=processor.feature_extractor,
            device=device,
            torch_dtype=torch_dtype,
            return_timestamps=True
        )

        print("Whisper model loaded successfully")

    def Transcribe(self, request, context):
        """
        Transcribe audio from the request.

        Args:
            request: TranscribeReq containing audio_data, file_name, lecture_name, and module
            context: gRPC context

        Returns:
            TranscribedRes containing the transcription with full text and chunks
        """
        try:
            # Extract request data
            audio_data = request.file_payload.audio_data
            file_name = request.file_payload.file_name
            lecture_name = request.lecture_name
            module = request.module
            # TODO timestamp offset for split values

            print(f"Transcribing file: {file_name} for lecture: {lecture_name}, module: {module}")

            # Create a temporary file to save the audio data
            with tempfile.NamedTemporaryFile(delete=False, suffix=os.path.splitext(file_name)[1]) as temp_file:
                temp_file.write(audio_data)
                temp_file_path = temp_file.name

            try:
                # Transcribe the audio
                result = self.pipe(temp_file_path, generate_kwargs={'language': 'german'})

                # Extract full text and chunks
                full_text = result['text']
                chunks = []

                for chunk_data in result['chunks']:
                    # Create timestamp object
                    timestamp = lecture_store_pb2.Timestamp(
                        timestamp_start=0.1 if chunk_data['timestamp'][0] == 0 else chunk_data['timestamp'][0],
                        timestamp_end=chunk_data['timestamp'][1]
                    )

                    # Create chunk object
                    chunk = whisper_pb2.Chunk(
                        text=chunk_data['text'],
                        timestamp=timestamp
                    )
                    chunks.append(chunk)

                # Create transcription object
                transcription = whisper_pb2.Transcription(
                    full_text=full_text,
                    chunks=chunks
                )

                # Create response
                response = whisper_pb2.TranscribedRes(payload=transcription)

                print(f"Transcription complete: {len(chunks)} chunks, {len(full_text)} characters")

                return response

            finally:
                # Clean up temporary file
                if os.path.exists(temp_file_path):
                    os.remove(temp_file_path)

        except Exception as e:
            print(f"Error during transcription: {e}", file=sys.stderr)
            context.set_code(grpc.StatusCode.INTERNAL)
            context.set_details(f"Error during transcription: {str(e)}")
            return whisper_pb2.TranscribedRes()


def serve():
    """Start the gRPC server."""
    port = os.environ.get("WHISPER_PORT", "50051")
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))

    whisper_grpc.add_WhisperServiceServicer_to_server(
        WhisperServiceServicer(), server
    )

    server.add_insecure_port(f'[::]:{port}')
    server.start()

    print(f"Whisper gRPC server started on port {port}")

    try:
        server.wait_for_termination()
    except KeyboardInterrupt:
        print("\nShutting down server...")
        server.stop(0)


if __name__ == '__main__':
    serve()
