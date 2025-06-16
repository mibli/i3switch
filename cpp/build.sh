#!/bin/bash
build_type=
install=

for i in "$@"; do
    case "$i" in
        rebuild)
            rm -rf build dist
            mkdir -p build
            ;;
        debug)
            build_type=Debug
            ;;
        release)
            build_type=Release
            ;;
        install)
            build_type=Release
            install=install
            ;;
        -h|--help|*)
            echo "Usage: ./build.sh (debug|release) [rebuild] [install]"
            exit 0
    esac
    shift
done

if [ -z "$build_type" ] ; then
    echo "Usage: ./build.sh (debug|release) [rebuild] [install]"
    exit 1
fi

: "${version:="0.0.0"}"
if [ "$version" = "0.0.0" ]; then
    git_version=$(git describe --tags --abbrev=0 --match="cpp-v[0-9]*" 2>/dev/null)
    if [ -z "$git_version" ]; then
        echo "Warning: No version tag found, using 0.0.0" >&2
    else
        version=${git_version#cpp-v}
    fi
fi

if [ -n "$build_type" ]; then
    echo "Building in $build_type mode"
    set -e
    mkdir -p build
    cmake -B build -DGIT_REPO_VERSION="$version" -DCMAKE_BUILD_TYPE="$build_type"
    make -C build
    if [ -n "$install" ]; then
        make -C build install
    fi
    set +e
fi

if [ "$build_type" = "Release" ]; then
    mkdir -p dist
    cp build/i3switch dist/i3switch || {
        echo "Error: Failed to move i3switch to dist directory" >&2
        exit 1
    }
fi

echo "Build completed successfully"
