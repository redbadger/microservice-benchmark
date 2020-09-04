FROM golang:1.15 AS build

COPY *.go ./

RUN go build -o api-go .

# ~~~~~~~~~~~~~~~~~~~~~~
FROM debian:buster-slim as release

RUN apt-get update && apt-get install -y \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd svc
USER svc

COPY --chown=svc --from=build \
    /go/api-go \
    /app/api-go

ENTRYPOINT ["/usr/bin/tini", "--"]

EXPOSE 3000
CMD ["/app/api-go"]
