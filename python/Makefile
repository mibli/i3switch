#!/bin/make

SOURCE_FILES := $(shell find src -name '*.py')
ROOT_DIR := $(shell realpath ..)
VERSION := $(shell $(ROOT_DIR)/scripts/version.sh python)
DEPENDENCIES := $(shell cat requirements.txt | tr '\n' '|')
.DEFAULT_GOAL := all

i3switch/version.py:
	@echo "i3switch version: $(VERSION)"
	sed "s|0.0.0|$(VERSION)|" i3switch/version.py.in > i3switch/version.py

.PHONY: deps
deps:
	pip freeze | grep -qE "$(DEPENDENCIES)" || echo "Missing dependencies: $(DEPENDENCIES)"
	@echo "All dependencies are satisfied."

dist/i3switch: deps i3switch/version.py $(SOURCE_FILES)
	pyinstaller --onefile -n i3switch run.py

.PHONY: all
all: dist/i3switch

.PHONY: clean
clean:
	rm -rf dist
	rm -rf build

.PHONY: install
install: dist/i3switch
	cp dist/i3switch /usr/local/bin/i3switch
