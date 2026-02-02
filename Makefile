# Makefile for chirality-runtime

.PHONY: all build build-release test lint fmt clean

all: build

# Build
build:
	cargo build

build-release:
	cargo build --release

# Test
test:
	cargo test --workspace

test-domain:
	cargo test -p chirality-domain

# Lint
lint:
	cargo clippy --workspace -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

# Clean
clean:
	cargo clean

# Development
dev:
	cargo run -p chirality-api

# Check (fast compile check)
check:
	cargo check --workspace

# Documentation
doc:
	cargo doc --workspace --no-deps --open
