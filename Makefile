default: backend-rust go java-blocking-io java-nonblocking-io node-ts rust

backend-rust: 
	cd legacy-backend-rust && make docker && cd manifests && make delete && make

java-blocking-io: 
	cd api-java-blocking-io && make docker && cd manifests && make delete && make

java-nonblocking-io: 
	cd api-java-nonblocking-io && make docker && cd manifests && make delete && make

node-ts: 
	cd api-node-ts && make docker && cd manifests && make delete && make

rust:
	cd api-rust && make docker && cd manifests && make delete && make

go: 
	cd api-go && make docker && cd manifests && make delete && make

.PHONY: backend-rust go java-blocking-io java-nonblocking-io node-ts rust
