.PHONY: default
default:
	go build . && \
		ulimit -n 4096 && \
		./api-go

.PHONY: docker
docker:
	tar cv Dockerfile -C .. api-go data | \
		docker build - \
			--tag api_go

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 api_go
