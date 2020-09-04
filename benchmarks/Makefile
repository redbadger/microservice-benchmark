.PHONY: default
default:
	export HOST=localhost CONCURRENCY=10 ITERATIONS=400 && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust.yaml

.PHONY: gke
gke:
	export HOST=TODO CONCURRENCY=200 ITERATIONS=4000 && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10 && \
		drill --stats --quiet --benchmark api-rust.yaml
