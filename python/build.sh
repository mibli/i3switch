#!/bin/bash
build=
install=false

for i in "$@"; do
    case "$i" in
        rebuild)
            rm -rf build i3switch.spec dist
            shift
            ;;
        debug)
            build="debug"
            shift
            ;;
        release)
            build="release"
            shift
            ;;
        install)
            install=true
            exit 0
            ;;
        -h|--help|*)
            echo "Usage: $0 (debug|release) [rebuild] [install]"
            exit 0
            ;;
    esac
done

: "${version:="0.0.0"}"
if [ "$version" = "0.0.0" ]; then
    git_version=$(git describe --tags --abbrev=0 --match="py-v[0-9]*" 2>/dev/null)
    if [ -z "$git_version" ]; then
        echo "Warning: No version tag found, using 0.0.0" >&2
    else
        version=${git_version#py-v}
        sed -i "s|0\.0\.0|$version|g" i3switch/__main__.py
    fi
fi

for dep in pyinstaller $(cat requirements.txt); do
    if pip freeze | grep -q "^$dep"; then
        echo "$dep is installed."
    else
        echo "Error: Dependency $dep is not missing." >&2
        echo "Please install it using pip or package manager."
        exit 1
    fi
done

if [ -n "$build" ]; then
    echo "Building i3switch in $build mode..."
    pyinstaller --onefile --name i3switch run.py || {
        echo "Error: Build failed." >&2
        exit 1
    }
fi

if [ "$install" = true ] && [ -f "build/bin/i3switch" ]; then
    echo "Installing i3switch..."
    sudo cp dist/i3switch /usr/local/bin/ || {
        echo "Error: Installation failed." >&2
        exit 1
    }
fi

echo "All tasks completed successfully."
