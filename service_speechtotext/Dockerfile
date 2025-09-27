FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY run.py .

EXPOSE 8003

ENTRYPOINT [ "python", "run.py" ]
