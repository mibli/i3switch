#!/bin/bash
builds=( )
rebuild=

die() {
    echo "[ERROR] $1"
    exit 1
}

rust_arg="rust:rust/target/x86_64-unknown-linux-gnu/release/i3switch"
python_arg="python:python/build/bin/i3switch"
cpp_arg="cpp:cpp/build/i3switch"

for i in "$@"; do
    case "$i" in
        rust)
            builds+=("$rust_arg")
            ;;
        python)
            builds+=("$python_arg")
            ;;
        cpp)
            builds+=("$cpp_arg")
            ;;
        all)
            builds+=("$rust_arg" "$python_arg" "$cpp_arg")
            ;;
        rebuild)
            rebuild=rebuild
            ;;
        *)
            echo "Unknown option: $i"
            echo "Usage: $0 ((rust|python|cpp)+|all) [rebuild]"
            exit 1
            ;;
    esac
done

if [ ${#builds[@]} -eq 0 ]; then
    echo "Usage: $0 ((rust|python|cpp)+|all) [rebuild]"
    exit 1
fi

for build in "${builds[@]}"; do
    build_dir=${build%%:*}
    build_path=${build##*:}

    echo "Building $build_dir..."

    (
        cd "$build_dir" || die "Failed to change directory to $build_dir"
        if ./build.sh -h | grep -q "test"; then
            ./build.sh test || die "Failed to build tests for $build_dir"
        fi
        ./build.sh release $rebuild || die "Failed to build $build_dir"
    ) && {
        echo "Build for $build_dir completed successfully, output binary can be found at $build_path"
    }
done
