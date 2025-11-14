FROM nvcr.io/nvidia/pytorch:25.08-py3-igpu

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

WORKDIR /app

COPY pyproject.toml uv.lock ./

RUN uv sync --frozen --no-dev

COPY target ./target
COPY embedding_server.py .

EXPOSE 8001

ENTRYPOINT ["uv", "run", "embedding_server.py"]
