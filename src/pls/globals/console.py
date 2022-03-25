from rich.console import Console

from pls.globals import args


def get_console():
    """
    Get a console instance. This console will record all output sent to it if the
    ``--export``/``-x`` flag is set via the CLI args.

    :return: a pre-configured ``rich.console.Console`` instance
    """

    return Console(record=args.args.export is not None)


console: Console
"""the Rich Console singleton"""
