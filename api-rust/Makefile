.PHONY: default
default:
	ulimit -n 4096 && \
		cargo run --release

.PHONY: docker
docker:
	docker build -t eu.gcr.io/feature-targeting/api_rust . && docker push

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 eu.gcr.io/feature-targeting/api_rust
