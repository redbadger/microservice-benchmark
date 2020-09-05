HOST=localhost
CONCURRENCY=10
TIME=30 # seconds
ITERATIONS=$(shell echo $$(($(CONCURRENCY) * $(TIME))) )

.PHONY: default
default:
	export HOST=$(HOST) CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust-tide.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust-actix.yaml

.PHONY: rust
rust:
	export HOST=$(HOST) CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-rust-tide.yaml
