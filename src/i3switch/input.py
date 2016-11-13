"""
Module handles input from udev
"""

import logging
import keyboard

HANDLERS = []

def bad_bind(action):
    """ if the function wasnt found, lets let them know """
    logging.warning("Bind: action doesnt exist: %s", action)

def good_bind(action, func, args):
    """ if the function was found, lets let them know """
    logging.info("Bind: executing action: %s", action)
    try:
        func(*args)
    except Exception as e:
        logging.error("Bind: %s threw an exception:", action, exc_info=True, stack_info=True)

def bind(control, combo, action, args):
    """ bind a combination to a function (taken by action name), and arguments """
    member = getattr(control, action, None)
    if not callable(member):
        func = lambda: bad_bind(action)
    else:
        func = lambda: good_bind(action, member, args)
    logging.info("Bind: bound %s to %s", combo, action)
    handler = keyboard.add_hotkey(combo, func)
    HANDLERS.append(handler)

def bind_all(control, config):
    """ bind all combinations from configuration """
    for combo, action in config.items():
        chunks = action.split(" ")
        (action, *args) = chunks
        bind(control, combo, action, args)
