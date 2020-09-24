NAME := nlsd
TEST_TARGET ?= chrome
BUILD_TARGET ?= bundler

build:
	wasm-pack build --target $(BUILD_TARGET)
	echo "`jq '.name="$(NAME)"' pkg/package.json`" > pkg/package.json

test:
	wasm-pack test --$(TEST_TARGET)
