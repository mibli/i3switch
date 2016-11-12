"""
Control focus movement commands
"""

import i3ipc
import logging
import sys

DIRECTIONS = ['left', 'up', 'right', 'down', 'parent']
TAB_DIRECTIONS = ['left', 'right']

class Controller:
    def __init__(self):
        self.i3 = i3ipc.Connection()
        self.wrap_tabs = True
        self.logger = logging.getLogger(self.__class__.__name__)
        self.logger.addHandler(logging.StreamHandler(sys.stdout))
        self.logger.setLevel(logging.INFO)

    # PRIVATE

    def get_root_context(self):
        return self.i3.get_tree()

    def get_focused_context(self):
        return self.get_root_context().find_focused()

    def get_tabbed_context(self, current_context=None):
        context = current_context or self.get_focused_context()
        try:
            while context.layout != 'tabbed':
                context = context.parent
        except Exception:
            logging.warning("Not in tabbed context")
            return None
        else:
            return context

    def get_next_context(self, parent_context):
        if parent_context is None:
            return None
        last = None
        for child in parent_context.descendents():
            if(last is not None and
               (last.focused or last.find_focused() is not None)):
                return child
            last = child
        if self.wrap_tabs:
            return parent_context.descendents()[0]
        return None

    def get_prev_context(self, parent_context):
        if parent_context is None:
            return None
        last = None
        for child in parent_context.descendents():
            if(last is not None and
               (child.focused or child.find_focused() is not None)):
                return last
            last = child
        if self.wrap_tabs:
            return parent_context.descendents()[-1]
        return None

    def send(self, command, *argv):
        formatted = command.format(*argv)
        self.logger.info(formatted)
        self.i3.command(formatted)

    # PUBLIC

    def switch(self, direction, parent=False):
        """ Switch in direction """
        context = self.get_focused_context()
        if direction not in DIRECTIONS:
            logging.error("%s is not one of %s", direction, str(DIRECTIONS))
            return
        if parent:
            tab_context = self.get_tabbed_context(context)
            if tab_context is not None:
                self.send('[con_id={}] focus', tab_context.id)
        self.send('focus {}', direction)

    def switch_tab(self, direction):
        if direction not in DIRECTIONS:
            logging.error("%s is not one of %s", direction, str(DIRECTIONS))
            return
        if direction not in TAB_DIRECTIONS:
            self.switch(direction)
        context = self.get_tabbed_context()
        tab = self.get_prev_context(context) if direction == 'left' else self.get_next_context(context)
        if tab is None:
            return
        self.send('[con_id={}] focus', tab.id)

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



