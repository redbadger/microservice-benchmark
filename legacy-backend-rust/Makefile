TAG=eu.gcr.io/feature-targeting/legacy_backend_rust

.PHONY: default
default:
	ulimit -n 4096 && \
		cargo run --release

.PHONY: docker
docker:
	tar cv \
		--exclude target \
		--exclude manifests \
		Dockerfile \
		-C .. \
			legacy-backend-rust \
			data | \
	docker build - \
		--tag $(TAG) && \
	docker push $(TAG)

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 $(TAG)
