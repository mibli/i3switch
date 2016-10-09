"""
Control focus movement commands
"""

import i3ipc
import logging
import sys

DIRECTIONS = ['left', 'up', 'right', 'down', 'parent']

class Controller:
    def __init__(self):
        self.i3 = i3ipc.Connection()
        self.logger = logging.getLogger(self.__class__.__name__)
        self.logger.addHandler(logging.StreamHandler(sys.stdout))
        self.logger.setLevel(logging.INFO)

    # PRIVATE

    def get_root_context(self):
        return self.i3.get_tree()

    def get_focused_context(self):
        return self.get_root_context().find_focused()

    def get_tabbed_context(self, current_context=None):
        context = current_context or get_focused_context()
        try:
            while context.layout != 'tabbed':
                context = context.parent
        except Exception:
            return None
        else:
            return context

    def send(self, command, *argv):
        formatted = command.format(*argv)
        self.logger.info(formatted)
        self.i3.command(formatted)

    # PUBLIC

    def switch(self, direction, parent=False):
        """ Switch in direction """
        context = self.get_focused_context()
        if parent:
            tab_context = self.get_tabbed_context(context)
            if tab_context is not None:
                self.send('[con_id={}] focus', tab_context.id)
        self.send('focus {}', direction)

    def switch_tab(self, direction):
        # tabs switching even if not parent
        pass

    def switch_to_tab(self, number):
        context = self.get_tabbed_context()
        try:
            tab = context.descendents()[number - 1]
        except Exception:
            self.logger.warning("There is no tab {}", number)
            return False
        else:
            self.send('[con_id={}] focus', tab.id)
            return True



