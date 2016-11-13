"""
Control focus movement commands
"""

import logging
import sys
from . import i3

DIRECTIONS = ['left', 'up', 'right', 'down', 'parent']
TAB_DIRECTIONS = ['prev', 'next']

class Controller:
    """ Our commands to control focus and windows, uses i3.I3 as i3 API """
    def __init__(self):
        self.i3 = i3.I3()
        self.logger = logging.getLogger(self.__class__.__name__)
        self.logger.addHandler(logging.StreamHandler(sys.stdout))
        self.logger.setLevel(logging.INFO)

    def switch(self, direction, group_tabs='nogroup'):
        """
        Switch in direction

        correct usage:
        >>> switch('up')
        >>> switch('right', 'group')
        """
        if direction not in DIRECTIONS:
            logging.error("%s is not one of %s", direction, str(DIRECTIONS))
            return
        if group_tabs not in ['group', 'nogroup']:
            logging.error("%s is not one of ['group', 'nogroup']", group_tabs)
            return

        focused = self.i3.focused()
        if group_tabs == 'group':
            tabs = self.i3.container(focused)
            if tabs is not None:
                self.i3.send('[con_id={}] focus', tabs.id)
        self.i3.send('focus {}', direction)

    def switch_tab(self, direction, wrap_tabs='nowrap'):
        """
        Switch tab to next, prev

        correct usage:
        >>> switch_tab('next')
        >>> switch_tab('prev', 'wrap')
        """
        if direction not in TAB_DIRECTIONS:
            logging.error("%s is not one of %s", direction, str(TAB_DIRECTIONS))
            return
        if wrap_tabs not in ['wrap', 'nowrap']:
            logging.error("%s is not one of ['wrap', 'nowrap']", wrap_tabs)
            return

        if direction == 'prev':
            tab = self.i3.prev_tab(wrap_tabs=(wrap_tabs == 'wrap'))
        else:
            tab = self.i3.next_tab(wrap_tabs=(wrap_tabs == 'wrap'))
        if tab is None:
            return
        self.i3.send('[con_id={}] focus', tab.id)

    def switch_to_tab(self, number):
        """
        Switch to nth tab

        correct usage:
        >>> switch_to_tab(1)
        >>> switch_to_tab(10)
        """
        tabs = self.i3.tabbed()
        try:
            tab = tabs.nodes[number - 1]
        except Exception:
            self.logger.warning("There is no tab %d", number)
            return False
        else:
            self.i3.send('[con_id={}] focus', tab.id)
            return True
