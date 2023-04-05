.PHONY: all build build-release build-production check test doc clean

# Default target
all: build test doc

build:
	@cargo build
	@cargo clippy

check:
	@cargo check
	@cargo clippy

clean:
	@cargo clean

build-release:
	@cargo build --release

build-production:
	@cargo build --profile production

test:
	@cargo test

doc:
	@cargo doc --no-deps
