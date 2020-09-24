TEST_ENV ?= chrome

build:
	wasm-pack build

test:
	wasm-pack test --$(TEST_ENV)
