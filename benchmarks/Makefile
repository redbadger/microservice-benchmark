HOST=localhost
CONCURRENCY=10
TIME=30 # seconds
ITERATIONS=$(shell echo $$(($(CONCURRENCY) * $(TIME))) )

.PHONY: default
default:
	export HOST=$(HOST) CONCURRENCY=$(CONCURRENCY) ITERATIONS=$(ITERATIONS) && \
		drill --stats --quiet --benchmark api-rust-actix.yaml
