## i3switch Rust version

### Requirements

Requirements will be usually managed by cargo itself, unless distribution is known
to manage Rust dependencies with packages manager, and `.cargo/config.<CODENAME>.toml`
is available for that system.

#### Debian Bookworm

To install requirements on debian bookworm, we use the package manager:

    sudo apt install           \
        librust-clap-3-dev     \
        librust-ctor-dev       \
        librust-log-dev        \
        librust-serde-json-dev \
        librust-simplelog-dev

#### Adding dependency restrictions

If target distro manages Rust packages and building with crates.io is not preferred,
a new restriction can be added by creating `.cargo/config.<SYSTEM_IDENTIFIER>.toml`
and extending `Makefile` `.cargo/config.toml` target with condition for the distro,
then documenting in a matching Requirements section in this README.

### Build

For most cases this should be enough:

    make

### Installation

The output binary will be located in `dist/i3switch`. You can install it manually,
or install to rust binary directory (must be in PATH to be usable) with:

    make install                                   # install to local rust binary directory
    sudo cp dist/i3switch /usr/local/bin/i3switch  # install to standard system directory

### Tests

To build tests just use make.

    make test
