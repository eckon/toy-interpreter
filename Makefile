.PHONY: run test markdown

run:
	cargo run

test:
	cargo test

markdown:
	npx prettier --write '**/*.md'
	npx markdownlint-cli '**/*.md' -f
