from rich.console import Console

from pls.globals import args


def get_console():
    """
    Get a console instance. This console will record all output sent to it if the
    ``--export``/``-x`` flag is set via the CLI args.

    :return: a pre-configured ``rich.console.Console`` instance
    """

    # Using ``getattr`` here to enable ``--update`` to use this console.
    return Console(record=getattr(args.args, "export", None) is not None)


console: Console
"""the Rich Console singleton"""
