#!/bin/bash
builds=( )
rebuild=

die() {
    echo "[ERROR] $1"
    exit 1
}

for i in "$@"; do
    case "$i" in
        rust|python|cpp)
            builds+=( "$i" )
            ;;
        all)
            builds+=( rust python cpp )
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
    echo "Building $build..."
    (
        cd "$build" || die "Failed to change directory to $build"
        if ./build.sh -h | grep -q "test"; then
            ./build.sh test || die "Failed to build tests for $build"
        fi
        ./build.sh release $rebuild || die "Failed to build $build"
    ) && {
        echo "Build for $build completed successfully, output binary can be found at $build/dist/i3switch"
    }
done
