TAG=eu.gcr.io/feature-targeting/api_java_blocking

.PHONY: default
default: build
	ulimit -n 4096 && \
		java -jar ./target/rest-service-0.0.1-SNAPSHOT.jar --server.port=8000

.PHONY: build
build:
	mvn clean package

.PHONY: docker
docker: build
	docker build -t $(TAG) . && docker push $(TAG)

.PHONY: docker-run
docker-run:
	docker run -p8000:8000 --ulimit nofile=4096:4096 $(TAG)
