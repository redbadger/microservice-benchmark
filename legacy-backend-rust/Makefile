.PHONY: default
default:
	ulimit -n 4096 && \
		cargo run --release

.PHONY: docker
docker:
	tar cv --exclude target Dockerfile -C .. legacy-backend-rust data | \
		docker build - \
			--tag legacy_backend_rust

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 legacy_backend_rust
