.DEFAULT_TARGET: all

.PHONY: all
all: server cli

.PHONY: server
server:
	cargo build --bin ironstore-server && ./target/debug/ironstore-server

.PHONY: clean
clean:
	rm -rf target

.PHONY: cli
cli:
	cargo build --bin ironstore-cli && ./target/debug/ironstore-cli

.PHONY: lint
lint:
	cargo clippy --all -- -D warnings
