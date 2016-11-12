#!/usr/bin/env python3

from argparse import ArgumentParser
from . import controller


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


if __name__ == "__main__":
    parser = ArgumentParser(description='Tabs-concious switching')
    group = parser.add_mutually_exclusive_group()
    group.add_argument('-d', '--direction', type=str,
                       choices=controller.DIRECTIONS,
                       help='Switch to [direction] of focused window (ignores tabs)')
    parser.add_argument('-t', '--tab', action='store_true',
                        help='For use with -d, to switch tabs')
    parser.add_argument('-p', '--parent', action='store_true',
                        help='For use with -d, to switch tabs')
    group.add_argument('-n', '--number', type=int,
                       help='Try to switch to n-th tab of focused container')
    args = parser.parse_args()

    def print_help_and_die(message):
        print(message)
        parser.print_help()
        exit(1)

    if args.direction is None:
        if args.tab:
            print_help_and_die("Use --tab only with -d or --direction")
        if args.parent:
            print_help_and_die("Use --parent only with -d or --direction")
    if args.number is not None and args.number <= 0:
        print_help_and_die("Tab indexes start from 1")

    i3 = controller.Controller()

    if args.number:
        i3.switch_to_tab(args.number)
    elif args.direction is not None:
        if args.tabs:
            i3.switch_tabs(args.direction)
        else:
            i3.switch(args.direction, args.parent)
    else:
        main(i3)
