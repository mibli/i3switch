"""
Configuration management
"""
import json
import os.path
import logging
from .utils import deep_update

DEFAULTS = {
    "bindings": {
        "Windows+h":              "switch left group",
        "Windows+l":              "switch right group",
        "Windows+k":              "switch up group",
        "Windows+j":              "switch down group",
        "Windows+Shift+h":        "move left",
        "Windows+Shift+l":        "move right",
        "Windows+Shift+k":        "move up",
        "Windows+Shift+j":        "move down",
        "Windows+Tab":            "switch_tab next wrap",
        "Windows+Shift+Tab":      "switch_tab prev nowrap"
    }
}

def find_and_load(path=None):
    """
    Check a few paths for config, and load if you find one

    by default retuns empty dictionary
    >>> find_and_load()
    {}
    """
    home = os.getenv("HOME")

    # searched in reverse order
    search_locations = [
        "/etc/i3-pyswitch/config.json",
        os.path.join(home, ".config/i3-pswitch/config.json")
    ]

    if path is not None:
        search_locations.append(path)

    for location in reversed(search_locations):
        try:
            with open(location, "r") as config_file:
                # TODO schema checking
                config_data = json.load(config_file)
            return config_data
        except Exception as e:
            logging.info("couldnt find config file under path\n%s\n%s", location, e)

    logging.error("couldnt find config file")
    return {}

def get_config(path=None):
    """
    Get config, applying default values
    """
    user_config = find_and_load(path)
    return deep_update(DEFAULTS, user_config)
