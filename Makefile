.PHONY: help

help: ## Show all commands
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup-infra: ## Setup infra
	docker compose -f compose.infra.yml up -d

setup-infra-down: ## Down infra
	docker compose -f compose.infra.yml down -v
