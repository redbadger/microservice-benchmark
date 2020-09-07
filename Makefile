default: backend-rust go java-blocking java-nonblocking node rust

backend-rust: 
	cd legacy-backend-rust && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

go: 
	cd api-go && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

java-blocking: 
	cd api-java-blocking-io && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

java-nonblocking: 
	cd api-java-nonblocking-io && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

node: 
	cd api-node-ts && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

rust:
	cd api-rust && $(MAKE) docker && cd manifests && $(MAKE) delete && $(MAKE)

all-no-build:
	cd legacy-backend-rust/manifests && $(MAKE) delete && $(MAKE)
	cd api-go/manifests && $(MAKE) delete && $(MAKE)
	cd api-java-blocking-io/manifests && $(MAKE) delete && $(MAKE)
	cd api-java-nonblocking-io/manifests && $(MAKE) delete && $(MAKE)
	cd api-node-ts/manifests && $(MAKE) delete && $(MAKE)
	cd api-rust/manifests && $(MAKE) delete && $(MAKE)

.PHONY: backend-rust go java-blocking java-nonblocking node rust all-no-build
