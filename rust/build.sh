#!/bin/bash
args=( )
build_type=

while [ -n "$1" ]; do
    case "$1" in
        rebuild)
            rm -rf target
            ;;
        debug)
            build_type=debug
            args+=(build)
            args+=(--debug)
            ;;
        test)
            build_type=test
            args+=(test)
            ;;
        release)
            build_type=release
            args+=(build)
            args+=(--release)
            args+=(--target x86_64-unknown-linux-gnu)
            ;;
        install)
            build_type=release
            args+=(install)
            args+=(--release)
            args+=(--root /usr/local)
            args+=(--target x86_64-unknown-linux-gnu)
            ;;
        -h|--help|*)
            echo "Usage: ./build.sh [(debug|release)] [rebuild] [install]"
            exit 0
    esac
    shift
done

: "${version:="0.0.0"}"
if [ "$version" = "0.0.0" ]; then
    git_version=$(git describe --tags --abbrev=0 --match="rust-v[0-9]*" 2>/dev/null)
    if [ -z "$git_version" ]; then
        echo "[WARN] No version tag found, using 0.0.0"
    else
        version=${git_version#rust-v}
        sed -i "s|0.0.0|$version|g" Cargo.toml
    fi
fi

if [ -n "${args[*]}" ]; then
    echo "Building in $build_type mode"
    cargo "${args[@]}" || exit 1
fi

echo "Build completed successfully"
