.PHONY: default
default:
	go build . && \
		ulimit -n 4096 && \
		./api-go

.PHONY: docker
docker:
	docker build --tag eu.gcr.io/feature-targeting/api_go . && docker push

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 eu.gcr.io/feature-targeting/api_go
