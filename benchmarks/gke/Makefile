CONCURRENCY=10
TIME=30 # seconds
ITERATIONS=$(shell echo $$(($(CONCURRENCY) * $(TIME))) )

default: java-blocking java-nonblocking node go rust

go:
	export CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-go.yaml && sleep 10

java-blocking:
	export CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-java-blocking.yaml && sleep 10

java-nonblocking:
	export CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-java-nonblocking.yaml && sleep 10

node:
	export CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-node.yaml && sleep 10

rust:
	export CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-rust.yaml && sleep 10

all:
	$(MAKE) default CONCURRENCY=10
	$(MAKE) default CONCURRENCY=10
	$(MAKE) default CONCURRENCY=10
	$(MAKE) default CONCURRENCY=10
	$(MAKE) default CONCURRENCY=10
	$(MAKE) default CONCURRENCY=20
	$(MAKE) default CONCURRENCY=20
	$(MAKE) default CONCURRENCY=20
	$(MAKE) default CONCURRENCY=20
	$(MAKE) default CONCURRENCY=20
	$(MAKE) default CONCURRENCY=50
	$(MAKE) default CONCURRENCY=50
	$(MAKE) default CONCURRENCY=50
	$(MAKE) default CONCURRENCY=50
	$(MAKE) default CONCURRENCY=50
	$(MAKE) default CONCURRENCY=100
	$(MAKE) default CONCURRENCY=100
	$(MAKE) default CONCURRENCY=100
	$(MAKE) default CONCURRENCY=100
	$(MAKE) default CONCURRENCY=100
	$(MAKE) default CONCURRENCY=200
	$(MAKE) default CONCURRENCY=200
	$(MAKE) default CONCURRENCY=200
	$(MAKE) default CONCURRENCY=200
	$(MAKE) default CONCURRENCY=200
	$(MAKE) default CONCURRENCY=500
	$(MAKE) default CONCURRENCY=500
	$(MAKE) default CONCURRENCY=500
	$(MAKE) default CONCURRENCY=500
	$(MAKE) default CONCURRENCY=500

.PHONY: go java-blocking java-nonblocking node rust
