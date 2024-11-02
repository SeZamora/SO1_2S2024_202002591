# Dockerfile del cliente con Fiber
FROM golang:1.23-bullseye AS build

WORKDIR /app

COPY . .

RUN go mod tidy

RUN go build -o atletismo .

EXPOSE 50052

CMD ["./atletismo"]
