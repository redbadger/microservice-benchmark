OVERLAY:=med-resources

.PHONY: default
default: install ## Install api-java-nonblocking

.PHONY: install
install: ## Install api-java-nonblocking
	kustomize build ./overlays/$(OVERLAY) | kubectl apply -f -

.PHONY: delete
delete: ## Remove api-java-nonblocking
	kustomize build ./overlays/$(OVERLAY) | kubectl delete -f - --ignore-not-found || true

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
