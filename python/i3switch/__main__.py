#!/usr/bin/env python3
""" i3 geometric window switcher

Usage:
  i3switch daemon
  i3switch (next | prev) [wrap]
  i3switch number <num>
  i3switch (left | up | right | down) [group]
  i3switch (-h | --help)

Options:
  next          Move focus to next tab/window
  prev          Move focus to previous tab/window
  number <num>  Move focus to tab/window number <num>
  right         Move focus right
  down          Move focus down
  left          Move focus left
  up            Move focus up
  -h --help     Show this help message

"""

from docopt import docopt
from . import controller


VERSION = '1.1.0'


def main(control):
    from .config import get_config
    from .input import bind_all
    from .utils import running
    from time import sleep
    import logging
    logging.basicConfig(level=logging.INFO)
    config = get_config()
    bind_all(control, config['bindings'])
    while running():
        sleep(1)


def entrypoint():
    """ Parses arguments and calls main() """
    arguments = docopt(__doc__, version='i3switch ' + VERSION)

    direction = None
    number = None
    daemon = None
    if arguments['next']:
        direction = 'right'
    elif arguments['prev']:
        direction = 'left'
    elif arguments['up']:
        direction = 'up'
    elif arguments['down']:
        direction = 'down'
    elif arguments['left']:
        direction = 'left'
    elif arguments['right']:
        direction = 'right'
    elif arguments['number']:
        try:
            number = int(arguments['<num>'])
        except ValueError:
            print("Invalid number: {}".format(arguments['<num>']))
            exit(1)
    elif arguments['daemon']:
        daemon = True
    else:
        print("Invalid command. Use -h or --help for usage information.")
        exit(1)

    group_tabs = 'nogroup'
    wrap = 'nowrap'
    if direction in ['up', 'down', 'left', 'right']:
        group_tabs = 'group' if arguments['group'] else 'nogroup'
    else:
        if arguments['wrap']:
            print("Wrap option is not applicable for this command")
            exit(1)
    if direction in ['next', 'prev']:
        wrap = 'wrap' if arguments['wrap'] else 'nowrap'
    else:
        if arguments['wrap']:
            print("Wrap option is not applicable for this command")
            exit(1)

    i3 = controller.Controller()

    if number:
        i3.switch_to_tab(number)
    elif direction is not None:
        if direction in ['next', 'prev']:
            i3.switch_tabs(direction, wrap)
        else:
            i3.switch(direction, group_tabs)
    elif daemon:
        main(i3)


if __name__ == "__main__":
    entrypoint()
