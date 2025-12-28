
format:
	cargo +nightly fmt

install:
	cargo install --path .

clippy:
	cargo clippy -- -D warnings

test:
	cargo test