CONCURRENCY=10
TIME=30
ITERATIONS=$(shell echo $$(($(CONCURRENCY) * $(TIME))) )

.PHONY: default
default:
	export HOST=localhost CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust.yaml

.PHONY: gke
gke:
	export HOST=TODO CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust.yaml
