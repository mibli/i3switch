#!/bin/bash

function die() {
    echo "$@"; exit 1
}

function check_min_version() {
    checked="$1"
    minimum="$2"
    lesser=$(printf "$checked\n$minimum" | sort -V | head -n1)
    [[ "$lesser" == "$minimum" ]] && return 0 || return 1
}

keyboard_version="$(python3 -m pip show keyboard | grep Version | cut -d' ' -f2)"
check_min_version "$keyboard_version" "0.9.3" || die "keyboard version $keyboard_version<0.9.3 requirement not met"
site_packages="$(python3 -c "from distutils.sysconfig import get_python_lib; print(get_python_lib())")"
set -e
cp -r src/i3switch/. $site_packages/i3switch
printf "#!/bin/bash\npython3 -m i3switch" > /usr/bin/i3switch
chmod +x /usr/bin/i3switch
