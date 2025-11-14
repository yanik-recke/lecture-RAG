FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

WORKDIR /app

COPY pyproject.toml uv.lock ./

RUN uv sync --frozen --no-dev

COPY target ./target
COPY whisper_server.py .

EXPOSE 50051

ENTRYPOINT ["uv", "run", "whisper_server.py"]
