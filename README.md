i3switch
===========

Application which aims to make some of the focus movement in i3 more intuitive.

It's initial version was written in python, however python was to slow to start,
so the control felt unresponsive, so I switched to c++

### What does it do

The main goal is to allow to move focus according to the visual space on the
screen, to resolve some of the annoyances of i3.

#### Tabs

One of the main adventages of tiling wms is ability have everythong You need to see simutalniously on single screen.
With that in mind, Your workflow probably involves moving around windows You
see.
That makes the way tabs work in i3 a bit counterproductive, because if You use
tabs, that's probably because You want to hide something that You don't
imidatiely want to see or use (maybe keep for later).
So Your focus movement should in first priority focus around the visual space
You have at hand. The way tabs work, they mess up Your workspace, unless You
want to dive into structures of the windows.

i3switch aims to resolve that issue, giving You a tool that allows to separate
tab switching from focus movement.

:pushpin: *EXAMPLE*

#### Uintuitive focus

One of another annoyances of i3 is that, when moving focus in direction, it
doesn't exactly go where You expect it to go.

:pushpin: *EXAMPLE*


### Current Features

* switch to nth tab of closest ancestor tabbed container

      i3switch -t -n TAB_ORDER

* switch to next/previous tab

      i3switch -t -d (LEFT|RIGHT)


### Planned features

* commands instead of options

      # I think these would be more readable and adequate
      i3switch tab next   # move focus to next tab
      i3switch tab 1      # move focus to 1st tab
      i3switch right      # move focus to right

* stacks support

      i3switch stack next # switch to next stack element
      i3switch stack 2    # switch to 2nd stack element
      i3switch any 3      # switch to 3rd tab or stack element

* visual space focus movement

  :pushpin: *EXAMPLE*

      i3switch right

### Building

    ./build.sh

or

    mkdir build
    cd build
    cmake -DCMAKE_BUILD_TYPE=Release  # this one might not be tested, I use Debug
    make

### Installation

:pushpin: *Proper CMake install*

    sudo cp build/i3switch /usr/local/bin/i3switch

### Depends

* i3
* nlohmann/json
