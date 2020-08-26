.PHONY: default
default: install ## Install legacy-backend-rust

.PHONY: install
install: ## Install legacy-backend-rust
	kustomize build . | kubectl apply -f -

.PHONY: delete
delete: ## Remove legacy-backend-rust
	kustomize build . | kubectl delete -f - --ignore-not-found || true

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
