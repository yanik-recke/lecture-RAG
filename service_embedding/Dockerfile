FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

WORKDIR /app

COPY requirements.txt .
COPY .env .
RUN pip install --no-cache-dir -r requirements.txt

EXPOSE 8001

COPY run.py .
