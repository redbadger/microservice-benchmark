FROM golang:1.15 AS build

COPY legacy-backend-go/server.go ./

RUN go build server.go

# ~~~~~~~~~~~~~~~~~~~~~~
FROM debian:buster-slim as release

RUN apt-get update && apt-get install -y \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd svc
USER svc

COPY --chown=svc --from=build \
    /go/server \
    /app/legacy-backend-go
COPY data /data

ENTRYPOINT ["/usr/bin/tini", "--"]

EXPOSE 3000
CMD ["/app/legacy-backend-go"]
