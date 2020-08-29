FROM rust:1.46 AS build

RUN rustup component add rustfmt clippy

WORKDIR /legacy-backend-rust
COPY legacy-backend-rust/Cargo.toml legacy-backend-rust/Cargo.lock ./

RUN mkdir -p ./src/ && echo 'fn main() {}' >./src/main.rs && echo '' >./src/lib.rs
RUN cargo build --release && rm -rf ./target/release/.fingerprint/legacy-backend-rust-*

COPY data ../data
COPY legacy-backend-rust/src ./src

RUN cargo clippy --release -- -D warnings && \
    cargo test --release && \
    cargo build --release

# ~~~~~~~~~~~~~~~~~~~~~~
FROM debian:buster-slim as release

RUN apt-get update && apt-get install -y \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd svc
USER svc

COPY --chown=svc --from=build \
    /legacy-backend-rust/target/release/legacy-backend-rust \
    /

ENTRYPOINT ["/usr/bin/tini", "--"]

EXPOSE 3000
CMD ["/legacy-backend-rust"]
