
format:
	cargo +nightly fmt

install:
	cargo install --path .

format-checks:
	cargo +nightly fmt --check

clippy:
	cargo clippy -- -D warnings

test:
	cargo test