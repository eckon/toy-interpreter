.PHONY: run test markdown

run:
	cargo run

test:
	cargo test -- --nocapture

markdown:
	npx prettier --write '**/*.md'
	npx markdownlint-cli '**/*.md' -f
