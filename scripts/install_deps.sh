#!/bin/bash
if [ "$1" = "python" ]; then
    sudo apt install -y python3-pip
    sudo pip3 install pyinstaller
    sudo pip3 install -r python/requirements.txt
elif [ "$1" = "cpp" ]; then
    sudo apt install -y \
        i3-wm \
        libdocopt0 \
        libdocopt-dev \
        nlohmann-json3-dev
elif [ "$1" = "rust" ]; then
    sudo apt install -y \
        cargo \
        libxcb1 \
        libxcb1-dev
elif [ "$1" = "package" ]; then
    sudo apt-get update
    sudo apt-get install -y ruby ruby-dev build-essential
    gem install --user-install fpm
fi
