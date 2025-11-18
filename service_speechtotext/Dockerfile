FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

LABEL org.opencontainers.image.authors="Yanik Recke <recke.yanik@outlook.de>"
LABEL org.opencontainers.image.version="3.0.0"
LABEL org.opencontainers.image.description="Transcription service running on Nvidia Orin Jetson ARM64 architecture"

RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY target ./target
COPY whisper_server.py .

# Add target directory to Python path
ENV PYTHONPATH="/app/target"

EXPOSE 50051

ENTRYPOINT ["python", "whisper_server.py"]