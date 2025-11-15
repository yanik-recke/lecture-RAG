# Backend-For-Frontend to Map REST Requests to Internal gRPC Communication


### Disclaimer
I did not focus on this part of the application much. My goal was to get a service mapping REST requests from
a potential frontend (which I would just generate using [v0](https://v0.app/) anyway) up and running
as fast as possible. Therefore, I used [Claude Code](https://www.claude.com/product/claude-code) for the entire
initial implementation. After reading
the corresponding *.proto* file, it generated all the 
appropriate method stubs and most of the implementations as well. While I did have to modify some of the code it did turn
out pretty good pretty fast. While rather skeptical of whether AI will meet the current (overhyped) expectations,
this is, in my opinion, one of the best use cases for AI. When you need to set 
up a pretty basic scaffolding for a project that you can later improve on.


## Development

## Environment Variables

| Variable               | Description                           | Default Value |
|------------------------|---------------------------------------|---------------|
| `LECTURE_SERVICE_HOST` | Hostname for the gRPC lecture service | `localhost`   |
| `LECTURE_SERVICE_PORT` | Port for the gRPC lecture service     | `40998`       |
| `SERVER_PORT`          | Port on which the BFF server runs     | `40999`       |

## Building and Running the Container

```shell
# Build
docker buildx build -f bff/Dockerfile --platform linux/amd64 -t bff .

# Run
docker run -p 40999:40999 --network lecture-rag-dev \
-e LECTURE_SERVICE_HOST=http://lecture-service \
-e LECTURE_SERVICE_PORT=40998 \
-e SERVER_PORT=40999
--rm \
bff
```

## Generating Sources
To generate the corresponding stubs and classes the *.proto* files need to available.  
Make sure that the *proto* plugin is configured with the correct path and run `mvn clean compile`  
