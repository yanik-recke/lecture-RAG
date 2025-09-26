## Development

Flask server running on port `8001`.

Building docker image: `docker build -t "embeddinggemma".\n
Running container on Jetson Orin Nano: `docker run --gpus all --ipc=host -v ~/.cache/huggingface/hub:/root/.cache/huggingface/hub --ulimit memlock=-1 --ulimit stack=67108864 -it --rm -p 8001:8001  embeddinggemma`.

The run command caches the model on the Jetson Orin Nano by specifying the volume with *-v* flag.