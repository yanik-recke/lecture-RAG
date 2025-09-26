FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY model.py .

EXPOSE 7860
