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

### Build features

The project provides option to enable or disable features. By default features that
are known to be stable are enabled, and features that are known to be unstable
are disabled. You can enable or disable features by setting `RUSTFLAGS` environment
variable before running `make`:

    # enable unstable features
    RUSTFLAGS="--all-features" \
        make
    # enable stable features
    RUSTFLAGS="-F i3,xcb --no-default-features" \
        make

#### Available features:

- `i3`: i3ipc-based backend for window switching (default)
- `xcb`: xcb-based backend for window switching (default)
- `wmctl`: wmctl-based backend for window switching (non-default)
