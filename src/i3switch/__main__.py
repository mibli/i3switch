#!/usr/bin/env python3

ARGS = {
    "settings": {
        '-d': "direction",
        '-n': "tab_number",
        '-c': "config"
    },
    "options": {
        '-t': 'tabs',
        '-p': 'parent'
    }
}


def print_help():
    output = []
    for setting in ARGS["settings"]:
        output += ["[", setting, " value]"]
    for option in ARGS["options"]:
        output += ["[", setting, "]"]
    print("".join(output))


def argget(args):
    options = []
    settings = {}
    argit = iter(argv)
    for arg in argit:
        try:
            key = ARGS["settings"][arg]
            value = next(argit)
            settings[key] = value
            continue
        except (KeyError, IndexError):
            pass
        try:
            options.append(ARGS["options"][arg])
            continue
        except KeyError:
            pass
    return options, settings


if __name__ == "__main__":
    from sys import argv
    options, settings = argget(argv)

    #from .controller import Controller
    #i3 = Controller()

    #if args.number:
    #    i3.switch_to_tab(args.number)
    #elif args.direction is not None:
    #    if args.tabs:
    #        i3.switch_tabs(args.direction)
    #    else:
    #        i3.switch(args.direction, args.parent)
    #else:
    #    from .config import get_config
    #    import keyboard
    #    config = get_config(args.config)
    #    for binding in config['bindings']:
    #        keyboard.add_
