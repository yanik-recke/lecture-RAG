FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu


WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY target ./target
COPY whisper_server.py .

# Add target directory to Python path
ENV PYTHONPATH="/app/target"

EXPOSE 50051

ENTRYPOINT ["python", "whisper_server.py"]
