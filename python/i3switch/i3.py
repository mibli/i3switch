"""
manage i3 context
"""

import logging
import i3ipc

WRAP_TABS = False

class I3:
    """
    Interfacing an interface because why not
    """
    def __init__(self):
        self.i3 = i3ipc.Connection()

    def send(self, command, *argv):
        """ send i3ipc command doing formatting """
        formatted = command.format(*argv)
        logging.info(formatted)
        self.i3.command(formatted)

    def root(self):
        """ get root context """
        return self.i3.get_tree()

    def focused(self):
        """ get focused context """
        return self.root().find_focused()

    def container(self, context=None, layouts=['tabbed', 'stacked']):
        """ get tabbed or stacked container """
        context = context or self.focused()
        try:
            while context.layout not in layouts:
                context = context.parent
        except Exception:
            logging.warning("Not in tabbed context")
            return None
        else:
            return context

    def next_tab(self, context=None, wrap_tabs=WRAP_TABS):
        """ get next tab in one of the containers """
        context = context or self.focused()
        context = self.container(context)
        if context is None:
            return None
        last = None
        children = context.nodes
        for child in children:
            if(last is not None and
               (last.focused or last.find_focused() is not None)):
                return child
            last = child
        if wrap_tabs:
            return children[0]
        return None

    def prev_tab(self, context=None, wrap_tabs=WRAP_TABS):
        """ same as next tab but reverses nodes """
        print(wrap_tabs)
        context = context or self.focused()
        context = self.container(context)
        if context is None:
            return None
        last = None
        children = reversed(context.nodes)
        for child in children:
            if(last is not None and
               (last.focused or last.find_focused() is not None)):
                return child
            last = child
        if wrap_tabs:
            return children[0]
        return None
