TAG=eu.gcr.io/feature-targeting/api_rust

.PHONY: default
default:
	ulimit -n 4096 && \
		cargo run --release

.PHONY: docker
docker:
	docker build -t $(TAG) . && docker push $(TAG)

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 $(TAG)
