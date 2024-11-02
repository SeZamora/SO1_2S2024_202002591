# Dockerfile del cliente con Fiber
FROM golang:1.23-bullseye AS build

WORKDIR /app

COPY . .

RUN go mod tidy

RUN go build -o ganadores .

CMD ["./ganadores"]
