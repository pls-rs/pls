import argparse


def add_args(parser: argparse.ArgumentParser):
    """
    Add meta arguments to the given parser.

    :param parser: the parser to which to add the arguments
    """

    meta = parser.add_argument_group(
        title="meta",
        description="meta-arguments for `pls` itself",
    )
    meta.add_argument(
        *["-h", "--help"],
        action="help",
        help="show this help message and exit",
    )
    meta.add_argument(
        *["-v", "--version"],
        action="version_update",
        help="show the version of the codebase",
    )
