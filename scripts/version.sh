#!/bin/bash

# This script retrieves the latest version of a specified programming language's package
# from git tags, formatted as "language-vX.Y.Z", where "language" is the short name
# (e.g., "cpp", "py", "rs") and "X.Y.Z" is the version number.
#
# Usage: ./version.sh <language>
#
# Example: ./version.sh cpp
#
# This script will output the version number or "0.0.0" if no version is found and an error message
# to stderr.

pattern=false && [ "$1" = "--pattern" ] && { pattern=true; shift; }

language=$1
case $language in
    cpp)
        language="cpp" ;;
    python|py)
        language="py" ;;
    rust|rs)
        language="rs" ;;
    *)
        echo "Unsupported language: $language"
        exit 1 ;;
esac

if $pattern; then
    echo "$language-v[0-9]*"
    exit 0
fi

version=$(
    git describe --tags --abbrev=0 --match "$language-v[0-9]*" 2>/dev/null |
        sed "s/^$language-v//"
    )

[ -z "$version" ] && {
    echo "No version found for $language, defaulting to 0.0.0" >&2
    version="0.0.0"
}
echo "$version"
