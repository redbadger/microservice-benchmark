.PHONY: default
default: build
	ulimit -n 4096 && \
		java -jar ./target/demo-0.0.1-SNAPSHOT.jar --server.port=8000

.PHONY: build
build:
	mvn clean package

.PHONY: docker
docker: build
	docker build -t api_java_nonblocking .

.PHONY: docker-run
docker-run:
	docker run -p8000:8000 --ulimit nofile=4096:4096 api_java_nonblocking
