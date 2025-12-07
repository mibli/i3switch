#!/bin/bash
language=$1
type=${2:-deb}
root_dir="$(realpath "$(dirname "$0")/..")"

case $language in
    cpp)
        language_long="cpp"
        language_short="cpp"
        ;;
    python|py)
        language_long="python"
        language_short="py"
        ;;
    rust|rs)
        language_long="rust"
        language_short="rs" ;;
    *)
        echo "Unsupported language: $language"
        exit 1
        ;;
esac
version=$("$root_dir"/scripts/version.sh $language_short)

conflict_flags=( )
for l in rs py cpp; do
    [[ "$l" != "$language_short" ]] &&
        conflict_flags+=( --conflicts "i3switch-$l" )
done

binary_path=$language_long/dist/i3switch
if [[ ! -f "$binary_path" ]]; then
    echo "Error: Binary not found at $binary_path"
    exit 1
fi

exec "$HOME"/.local/share/gem/ruby/*/bin/fpm -s dir -t "$type" \
    -n "i3switch-$language_short" \
    -v "$version-ubuntu" \
    --description "i3 advanced window switching ($language_long version)" \
    --maintainer "Miłosz Bliźniak <mibli@gmx.com>" \
    --architecture amd64 \
    "${conflict_flags[@]}" \
    "$binary_path"=/usr/bin/i3switch
