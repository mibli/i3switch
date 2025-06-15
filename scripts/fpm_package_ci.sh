#!/bin/bash
language=$1
binary_path=$2
type=${3:-deb}
version=$(git describe --tags --abbrev=0 --match "$language-v[0-9]*" 2>/dev/null |
    sed "s/^$language-v//")
[ -z "$version" ] && version="0.0.0"

conflict_flags=( )
for i in cpp python rust; do
    [[ "$language" != "$i" ]] && conflict_flags+=( --conflicts "i3switch-$i" )
done

exec "$HOME"/.local/share/gem/ruby/*/bin/fpm -s dir -t "$type" \
    -n "i3switch-$language" \
    -v "$version" \
    --description "i3 advanced window switching ($language version)" \
    --maintainer "Miłosz Bliźniak <mibli@example.com>" \
    --architecture amd64 \
    "${conflict_flags[@]}" \
    "$binary_path"=/usr/bin/i3switch
