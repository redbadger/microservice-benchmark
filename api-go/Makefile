TAG=eu.gcr.io/feature-targeting/api_go

.PHONY: default
default:
	go build . && \
		ulimit -n 4096 && \
		./api-go

.PHONY: docker
docker:
	docker build --tag $(TAG) . && docker push $(TAG)

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 $(TAG)
