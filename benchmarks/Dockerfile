FROM rust:1.46

RUN cargo install drill

COPY gke/*.yaml /benchmarks/
COPY gke/Makefile /benchmarks/

CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"
