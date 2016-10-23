"""
Module handles input from udev
"""

import threading

from Xlib.display import Display
from Xlib import X

from utils import running

class Keyboard(threading.Thread):
    def __init__(self, defs=[]):
        threading.Thread.__init__()
        self.display = Display()
        self.root = self.display.screen().root
        self.defs = []

        self.root.change_attributes(event_mask=X.KeyPressMask)

        for keycode, callback in defs:
            self.add_key(keycode, callback)

    def get_callback(self, key):
        for keycode, callback in self.defs:
            if keycode == key:
                return callback
        else:
            return None

    def handle_event(self, xevent):
        if xevent.type != X.KeyPress:
            return
        callback = get_callback(xevent.detail)
        if callback is not None:
            callback()

    def add_key(self, keycode, callback):
        if get_def(keycode) is not None:
            return False
        self.defs.append((keycode, callback))
        self.root.grab_key(keycode, X.AnyModifier, 1, X.GrabModeAsync, X.GrabModeAsync)
        return True

    def add_keysym(self, keysym, callback):
        """ does same as add_key, but uses names like Control_L instead of keycodes """
        keycode = self.display.keysym_to_keycode(keysym)
        self.add_key(keycode, callback)

    def run(self):
        while running():
            xevent = self.root.display.next_event()
            self.handle_event(xevent)
