#!/bin/sh
[ $1 == "rebuild" ] && {
    rm -rf build
    mkdir -p build
}
pushd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
make
popd
