APP_NAME := whimsy
COVERAGE := 'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'

# setup
setup\:vendor: # Install cargo vendor and run it
	@mkdir -p .cargo
	@which cargo-vendor >/dev/null 2>&1 || cargo install \
		cargo-vendor --force
	@cargo vendor > .cargo/config
.PHONY: setup\:vendor

setup\:tool: # Install development tools
	@mkdir -p .git/hooks
.PHONY: setup\:tool

setup\:all: setup\:tool setup\:vendor # Setup vendor and tool both
.PHONY: setup\:all

setup: setup\:vendor # Alias of setup:vendor
.PHONY: setup

# vet
vet\:check: # Check Rust syntax [synonym: check]
	@cargo check --all --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Check format without changes [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Check style using clippy [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: vet\:check vet\:format vet\:lint # Check code using all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias of vet:check
.PHONY: vet

# test
test\:bin: # Run tests for application binary
	@cargo test --bin $(APP_NAME)
.PHONY: test\:bin

test\:doc: # Run doc tests
	@cargo test --doc
.PHONY: test\:doc

test\:all: test\:doc test\:bin # Run tests for doc and application
.PHONY: test\:all

test: test\:bin # Alias of test:bin
.PHONY: test

# coverage
_get_covered:
	result=($(DST_DIR)/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$(DST_DIR)/index.js*"; \
	fi; \
	file=($(DST_DIR)/debug/deps/$(APP_NAME)-*); \
	kcov --verify --include-path=$(SRC_DIR) $(DST_DIR) $${file[0]}; \
	grep 'index.html' $(DST_DIR)/index.js* | \
		grep --only-matching --extended-regexp $(COVERAGE)

coverage\:bin: # Get coverage of tests for application binary [synonym: cov:bin]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/bin"; \
	cargo test --bin $(APP_NAME) --no-run --target-dir=$${target_dir}; \
	make -s SRC_DIR=$${dir}/src DST_DIR=$${target_dir} _get_covered
.PHONY: coverage\:bin

cov\:bin: coverage\:bin
.PHONY: cov\:bin

coverage\:all: coverage\:bin # Get coverage from all tests [synonym: cov:all]
.PHONY: coverage\:all

cov\:all: | coverage\:all
.PHONY: cov\:all

coverage: coverage\:bin # Alias of coverage:bin [synonym: cov]
.PHONY: cov

cov: coverage
.PHONY: cov

# build
build\:debug: # Run debug build
	@cargo build --bin $(APP_NAME)
.PHONY: build\:debug

build: build\:debug # Alias of build:debug
.PHONY: build

build\:release: # Build release arfitacts
	cargo build --bin $(APP_NAME) --release
.PHONY: build\:release

# utility
clean: # Remove cache and built artifacts
	@cargo clean
.PHONY: clean

install:
	@cargo install --path $(APP_NAME) --force
.PHONY: install

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = build
default: build
