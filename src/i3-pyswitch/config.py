import json
import os.path

def find_and_load(path=None):
    home = os.getenv("HOME")
    # searched in reverse order
    search_locations = (
        "/etc/i3-pyswitch/config.json",
        os.path.join(home, ".config/i3-pswitch/config.json")
    )
    if path is not None:
        search_locations.append(path)
    for location in reversed(search_locations):
        try:
            with open(location, "r") as config_file:
                # TODO schema checking
                config_data = json.load(config_file)
            return config_data
        except:
            logging.info("couldnt find config file under path\n%s", location)
    logging.error("couldnt find config file")
    return {}

def get_defaults():
    return {
        "bindings": {
            "left":     ["Mod4", "h"],
            "right":    ["Mod4", "l"],
            "top":      ["Mod4", "k"],
            "bottom":   ["Mod4", "j"],
            "tab-next": ["Mod4", "Tab"],
            "tab-prev": ["Mod4", "Shift", "Tab"]
        }
    }

def get_config(path=None):
    defaults = get_defaults()
    user_config = find_and_load(path)
    return deep_update(defaults, user_config)
