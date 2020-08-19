.PHONY: default
default:
	cargo run

.PHONY: docker
docker:
	docker build -t legacy_backend_rust .

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 legacy_backend_rust
