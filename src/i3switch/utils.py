"""
Various helpful utils for programming
"""

import copy


def areinstances(items, types):
    """
    isinstance for multiple items

    >>> areinstances(({}, {"asd": True}), dict)
    True
    >>> areinstances(({}, []), dict)
    False
    """
    return all(isinstance(item, types) for item in items)


def deep_update(A, B, inplace=False, extend=False):
    """
    same as dict.update, but it navigates embedded dictionaries and updates them too
    instead of simply overwriting them

    Some definitions first
    >>> A = {"a": {"b": {"c": ["abc"]}, "d": True}, "e": 20}
    >>> B = {"a": {"b": {"c": "bca"}}}

    You can deep update A with B, and return C without changing either
    >>> deep_update(
    ...     A, B
    ... ) == {'a': {'d': True, 'b': {'c': 'bca'}}, 'e': 20}
    True
    >>> A == {'a': {'d': True, 'b': {'c': ['abc']}}, 'e': 20}
    True

    You can tell function to replace values in the current dictionary
    >>> deep_update(
    ...     A, B, inplace=True
    ... ) == {'a': {'d': True, 'b': {'c': 'bca'}}, 'e': 20}
    True
    >>> A == {'a': {'d': True, 'b': {'c': 'bca'}}, 'e': 20}
    True

    Cleanup
    >>> del A, B
    """
    C = A if inplace else copy.copy(A)
    commons = set(A.keys()).intersection(B.keys())
    for common in commons:
        a, b = C[common], B[common]
        if areinstances((a, b), dict):
            C[common] = deep_update(a, b, inplace, extend)
        elif extend:
            if areinstances((a, b), list):
                # todo allow specyfying action for lists
                a.extend(b)
            elif areinstances((a, b), set):
                # todo allow specyfying action for sets
                a.union(b)
        else:
            C[common] = b
    return C


def deep_get(dictionary, keys, default=None):
    """
    get key value by the path given in form of keys sequence

    Some definitions first
    >>> A = {"a": {"b": {"c": ["abc"]}, "d": True}, "e": 20}

    You can specify path and get value under it
    >>> deep_get(A, ["a", "b", "c"])
    ['abc']
    >>> deep_get(A, ["a", "d"])
    True
    >>> deep_get(A, ["e"])
    20

    If it cannot be found it will return None or a default value specified
    >>> deep_get(A, ["a", "d", "f"])
    >>> deep_get(A, ["a", "d", "f"], 0)
    0

    Input dictionary should not change under any circumstances
    >>> A == {"a": {"b": {"c": ["abc"]}, "d": True}, "e": 20}
    True

    Cleanup
    >>> del A
    """
    try:
        value = dictionary
        for key in keys:
            value = value[key]
    except (KeyError, TypeError):
        value = default
    return value


class Runtime:
    """
    class for management of application runtime state, handles exit signals automatically
    """
    def __init__(self, sigs=None):
        import signal
        from threading import Event

        self.run_event = Event()
        self.run_event.set()
        sigs = sigs or (signal.SIGPIPE, signal.SIGINT, signal.SIGTERM, signal.SIGABRT)
        for sig in sigs:
            signal.signal(sig, self.stop)

    def stop(self, *_):
        """ set as not running """
        self.run_event.clear()

    def running(self):
        """ return running status """
        return self.run_event.is_set()

GLOBAL_RUNTIME = Runtime()

def running():
    """
    Function returns running status. Which is definied by automatically created signal handler

    >>> running()
    True
    >>> GLOBAL_RUNTIME.stop()
    >>> running()
    False
    """
    # return status
    return GLOBAL_RUNTIME.running()

if __name__ == "__main__":
    import doctest
    doctest.testmod()
