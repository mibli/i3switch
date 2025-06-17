#!/bin/make

SOURCE_FILES := $(shell find src -name '*.rs')
ROOT_DIR := $(shell realpath ..)
VERSION := $(shell $(ROOT_DIR)/scripts/version.sh rs)
.DEFAULT_GOAL := all

Cargo.toml: Cargo.toml.in
	@echo "i3switch version: $(VERSION)"
	sed "s|0.0.0|$(VERSION)|" Cargo.toml.in > Cargo.toml

target/debug/i3switch: Cargo.toml $(SOURCE_FILES)
	cargo build

target/release/i3switch: Cargo.toml $(SOURCE_FILES)
	cargo build --release

target/x86_64-unknown-linux-gnu/release/i3switch: Cargo.toml $(SOURCE_FILES)
	cargo build --release --target x86_64-unknown-linux-gnu

dist/i3switch: target/x86_64-unknown-linux-gnu/release/i3switch
	mkdir -p dist
	cp target/x86_64-unknown-linux-gnu/release/i3switch dist/

.PHONY: test
test: $(SOURCE_FILES)
	cargo test

.PHONY: all
all: dist/i3switch

.PHONY: clean
clean:
	rm -rf dist
	rm -rf target

.PHONY: install
install: target/release/i3switch
	cargo install --path .
