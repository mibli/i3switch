"""
Module handles input from udev
"""

import logging
import keyboard

HANDLERS = []

def bad_bind(action):
    """ if the function wasnt found, lets let them know """
    logging.warning("Bind: action doesnt exist: %s", action)

def good_bind(action, function, args):
    """ if the function was found, lets let them know """
    logging.info("Bind: executing action: %s", action)
    function(*args)

def bind(control, combo, action, args):
    """ bind a combination to a function (taken by action name), and arguments """
    function = getattr(control, action, None)
    if not callable(function):
        function = lambda: bad_bind(action)
    else:
        function = lambda: good_bind(action, function, args)
    logging.info("Bind: bound %s to %s", combo, action)
    handler = keyboard.add_hotkey(combo, function)
    HANDLERS.append(handler)

def bind_all(control, config):
    """ bind all combinations from configuration """
    for combo, action in config.items():
        chunks = action.split(" ")
        (action, *args) = chunks
        bind(control, combo, action, args)
