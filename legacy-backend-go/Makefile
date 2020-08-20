.PHONY: default
default:
	go build . && \
		ulimit -n 4096 && \
		./legacy-backend-go

.PHONY: docker
docker:
	tar cv Dockerfile -C .. legacy-backend-go data | \
		docker build - \
			--tag legacy_backend_go

.PHONY: docker-run
docker-run:
	docker run -p3000:3000 --ulimit nofile=4096:4096 legacy_backend_go
