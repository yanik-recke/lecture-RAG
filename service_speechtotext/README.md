

## Development

Generate messages and stubs with (you might have to create the target/ directory):
```shell
# Run from service_speechtotext directory
python3 -m grpc_tools.protoc \
  -I../proto \
  --python_out=./target \
  --grpc_python_out=./target \
  --pyi_out=./target \
  ../proto/whisper_service.proto \
  ../proto/lecture_service.proto \
  ../proto/lecture_store.proto
```

To run on Jetson Orin Nano use `docker run --gpus all --ipc=host -v ~/.cache/huggingface/hub:/root/.cache/huggingface/hub --ulimit memlock=-1 --ulimit stack=67108864 -it --rm -p 7860:7860 s2t`

### Server
The gRPC server listens on port 50051.

### Testing
To get a quick test gradio server running check out [test](test).  
```shell
cd test
docker build -t "s2t" .

# Change the volume path to a directory on host machine
docker run --gpus all --ipc=host -v ~/.cache/huggingface/hub:/root/.cache/huggingface/hub -v ~/PATH/TO/FOLDER/ON/HOST:/app/transcriptions --ulimit memlock=-1 --ulimit stack=67108864 -it --rm -p 7860:7860 s2t
```

## Issues

[_] Change Transcribe request to include a timestamp offset to allow for splitting
audios and uploading them separately while still ensuring correct timestamps