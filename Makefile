
all: help
# Run the program (usage: make run ARGS="parse -f list.txt")
run:
	cargo run $(ARGS)

example:
	cargo run -- parse -f examples/shopping_list.txt --verbose

test:
	cargo test

check: fmt clippy test
	@echo "All checks passed!"

fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean

help:
	@echo "Available commands:"
	@echo "  make run ARGS=\"...\"  - Run program with arguments"
	@echo "  make example          - Run with example shopping list"
	@echo "  make test            - Run tests"
	@echo "  make check           - Run format, clippy and tests"
	@echo "  make fmt             - Format code"
	@echo "  make clippy          - Run clippy lints"
	@echo "  make clean           - Clean build files"