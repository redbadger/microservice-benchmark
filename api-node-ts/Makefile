.PHONY: default
default:
	npm run build
	npm run start	

.PHONY: docker
docker:
	docker build --tag eu.gcr.io/api_node . && docker push

.PHONY: docker-run
docker-run:
	docker run -p8000:8000 --ulimit nofile=4096:4096 eu.gcr.io/api_node
