.PHONY: default
default: operator profile config ## Install Istio

.PHONY: operator
operator: ## Install Istio Operator
	istioctl operator init
	@until kubectl wait --for=condition=ready pod -l name=istio-operator -n istio-operator --timeout=480s; \
	do \
		echo "Waiting for istio operator"; \
		sleep 10 ;\
	done

.PHONY: profile
profile: ## Install Istio profile
	kustomize build 1-profile | kubectl apply -f -
	@until kubectl wait --for=condition=ready pod -l app=istiod -n istio-system --timeout=480s; \
	do \
		echo "Waiting for istiod"; \
		sleep 10 ;\
	done

.PHONY: config
config: ## Install Istio config
	kustomize build 2-config | kubectl apply -f -

.PHONY: delete
delete: ## Remove Istio
	kubectl delete istiooperators.install.istio.io -n istio-system istiocontrolplane --ignore-not-found || true
	istioctl operator remove || true
	kustomize build 2-config | kubectl delete -f - --ignore-not-found || true
	kustomize build 1-profile | kubectl delete -f - --ignore-not-found || true

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
