#!/bin/make

SOURCE_FILES := $(shell find src -name '*.cpp' -or -name '*.hpp')
ROOT_DIR := $(shell realpath ..)
VERSION := $(shell $(ROOT_DIR)/scripts/version.sh cpp)
.DEFAULT_GOAL := all

build/debug: $(SOURCE_FILES) CMakeLists.txt
	cmake -B build/debug -S . \
		-DCMAKE_BUILD_TYPE=Debug \
		-DGIT_REPO_VERSION=$(VERSION)

build/release: $(SOURCE_FILES) CMakeLists.txt
	cmake -B build/release -S . \
		-DCMAKE_BUILD_TYPE=Release \
		-DGIT_REPO_VERSION=$(VERSION)

build/debug/i3switch: build/debug
	make -C build/debug

build/release/i3switch: build/release
	make -C build/release

dist/i3switch: build/release/i3switch
	mkdir -p dist
	cp build/release/i3switch dist/i3switch

.PHONY: all
all: dist/i3switch

.PHONY: clean
clean:
	rm -rf dist
	rm -rf build

.PHONY: install
install: target/release/i3switch
	make -C target/release install
