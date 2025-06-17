#!/bin/make
CPP_VERSION := $(shell scripts/version.sh cpp)
PYTHON_VERSION := $(shell scripts/version.sh python)
RUST_VERSION := $(shell scripts/version.sh rust)
.DEFAULT_GOAL := all

%/dist/i3switch:
	make -C $* dist/i3switch

%/dist/CHANGELOG.md:
	git cliff -c $*/cliff.toml -o $*/dist/CHANGELOG.md \
		--tag-pattern "$(shell scripts/version.sh --pattern $*)"

i3switch-cpp-${CPP_VERSION}: cpp/dist/i3switch
	scripts/fpm_package_ci.sh cpp deb

i3switch-py-${PYTHON_VERSION}: python/dist/i3switch
	scripts/fpm_package_ci.sh python deb

i3switch-rs-${RUST_VERSION}: rust/dist/i3switch
	scripts/fpm_package_ci.sh rust deb

.PHONY: all
all: i3switch-cpp-${CPP_VERSION} i3switch-py-${PYTHON_VERSION} i3switch-rs-${RUST_VERSION}

.PHONY: clean
clean:
	rm -rf */build
	rm -rf */dist
	rm -f i3switch-*.deb
