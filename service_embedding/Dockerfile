FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

LABEL org.opencontainers.image.authors="Yanik Recke <recke.yanik@outlook.de>"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.description="Embedding service running on Nvidia Orin Jetson ARM64 architecture"

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY target ./target
COPY embedding_server.py .

# Add target directory to Python path
ENV PYTHONPATH="/app/target"

EXPOSE 50052

ENTRYPOINT ["python", "embedding_server.py"]