"""
Various helpful utils for programming
"""

def running(sigs=None):
    """
    Function returns running status. Which is definied by automatically created signal handler

    >>> running()
    False
    """
    if not hasattr(running, "run"):
        import signal

        # running handler
        running.run = True
        def stop(_, __):
            running.run = False

        # signals registration
        sigs = sigs or (signal.SIGPIPE, signal.SIGINT, signal.SIGTERM, signal.SIGKILL)
        for sig in sigs:
            signal.signal(sig, stop)

    # return status
    return running.run

