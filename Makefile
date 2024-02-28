.DEFAULT_TARGET: all

.PHONY: all
all: server cli

.PHONY: server
server:
	cargo build

.PHONY: clean
clean:
	rm -rf target

.PHONY: cli
cli:
	cargo build

.PHONY: lint
lint:
	cargo clippy --all -- -D warnings
