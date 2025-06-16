#!/bin/bash
language=$1
type=${2:-deb}

declare -A languages_short=(
    [cpp]="cpp"
    [python]="py"
    [rust]="rs"
)
language_short="${languages_short[$language]}"
version=$(git describe --tags --abbrev=0 --match "$language_short-v[0-9]*" 2>/dev/null |
    sed "s/^$language_short-v//")
[ -z "$version" ] && version="0.0.0"

conflict_flags=( )
for language_long in "${!languages_short[@]}"; do
    [[ "$language" != "$language_long" ]] &&
        conflict_flags+=( --conflicts "i3switch-${languages_short[$language_long]}" )
done

binary_path=$language/build/i3switch
if [[ ! -f "$binary_path" ]]; then
    echo "Error: Binary not found at $binary_path"
    exit 1
fi

exec "$HOME"/.local/share/gem/ruby/*/bin/fpm -s dir -t "$type" \
    -n "i3switch-$language_short" \
    -v "$version" \
    --description "i3 advanced window switching ($language version)" \
    --maintainer "Miłosz Bliźniak <mibli@example.com>" \
    --architecture amd64 \
    "${conflict_flags[@]}" \
    "$binary_path"=/usr/bin/i3switch
