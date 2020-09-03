.PHONY: default
default:
	ulimit -n 4096 && \
		mvn clean package && \
		java -jar ./target/rest-service-0.0.1-SNAPSHOT.jar --server.port=8000

.PHONY: docker
docker:
	docker build -t api_java_blocking .

.PHONY: docker-run
docker-run:
	docker run -p8000:8000 --ulimit nofile=4096:4096 api_java_blocking
