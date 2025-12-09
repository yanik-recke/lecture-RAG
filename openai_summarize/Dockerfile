FROM python:3.13-slim

LABEL org.opencontainers.image.authors="Yanik Recke <recke.yanik@outlook.de>"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.description="Summary service using OpenAI API"

WORKDIR /app

# Copy requirements
COPY pyproject.toml .

# Install dependencies
RUN pip install --no-cache-dir -e .

# Copy application files
COPY target ./target
COPY summary_server.py .

# Add target directory to Python path
ENV PYTHONPATH="/app"

EXPOSE 50053

ENTRYPOINT ["python", "summary_server.py"]
