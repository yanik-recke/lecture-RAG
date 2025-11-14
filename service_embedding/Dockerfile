FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

WORKDIR /app

COPY target ./target
COPY embedding_server.py .

# Add target directory to Python path
ENV PYTHONPATH="/app/target"

EXPOSE 50052

ENTRYPOINT ["python", "embedding_server.py"]
