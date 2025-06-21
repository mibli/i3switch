#!/bin/make

SOURCE_FILES := $(shell find src -name '*.rs')
ROOT_DIR := $(shell realpath ..)
VERSION := $(shell $(ROOT_DIR)/scripts/version.sh rs)
.DEFAULT_GOAL := all
OS_CODENAME=$(shell sed -e '/^VERSION_CODENAME/!d' -e 's/^.*=//' /etc/os-release)

.cargo/config.toml:
ifeq ($(OS_CODENAME),bookworm)
	ln -s config.bookworm.toml .cargo/config.toml
else
	echo > .cargo/config.toml
endif

Cargo.toml: Cargo.toml.in .cargo/config.toml
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
test: Cargo.toml $(SOURCE_FILES)
	cargo test

.PHONY: all
all: dist/i3switch

.PHONY: clean
clean:
	rm -rf dist
	rm -rf target
	rm -f Cargo.toml
	rm -f Cargo.lock
	rm -f .cargo/config.toml

.PHONY: install
install: target/release/i3switch
	cargo install --path .
