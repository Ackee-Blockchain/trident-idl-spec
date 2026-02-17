
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

release-workspace:
	cargo workspaces publish --token $(TOKEN) --publish-as-is