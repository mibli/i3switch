#!/bin/sh
while [ -n "$1" ]; do
    case "$1" in
        "rebuild")
            rm -rf build
            mkdir -p build
            ;;
        "debug")
            build_type="Debug"
            ;;
        "release")
            build_type="Release"
            ;;
        ?)
            echo "Usage: ./build.sh [(debug|release)] [rebuild]"
    esac
    shift
done

: ${build_type="Release"}
: ${install:=false}

mkdir -p build
pushd build
cmake .. -DCMAKE_BUILD_TYPE="$build_type"
make
popd
