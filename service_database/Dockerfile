# Build stage
FROM golang:1.24-alpine AS builder
LABEL authors="Yanik Recke"

RUN apk add --no-cache git

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o /app/main ./main.go

# Final stage
FROM scratch

COPY --from=builder /app/main /main
#COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENTRYPOINT ["/main"]