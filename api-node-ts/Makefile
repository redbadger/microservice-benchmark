.PHONY: default
default:
	npm run build
	npm run start	

.PHONY: docker
docker:
	docker build --tag api_node .

.PHONY: docker-run
docker-run:
	docker run -p8000:8000 --ulimit nofile=4096:4096 api_node
