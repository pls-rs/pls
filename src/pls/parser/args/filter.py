import argparse
import re

from pls.parser.actions import BooleanOptionalAction, StoreOrCountAction


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for filtering to the given parser.

    :param parser: the parser to which to add the arguments
    """

    filtering = parser.add_argument_group(
        title="filtering",
        description="arguments used for filtering nodes in the output",
    )
    filtering.add_argument(
        *["-a", "--all"],
        action=StoreOrCountAction,
        help="increasingly show low-importance files that would otherwise be hidden",
    )
    filtering.add_argument(
        "--dirs",
        action=BooleanOptionalAction,
        help="[underline]show[/]/[magenta]hide[/] directories in the output",
    )
    filtering.add_argument(
        "--files",
        action=BooleanOptionalAction,
        help="[underline]show[/]/[magenta]hide[/] files in the output",
    )
    filtering.add_argument(
        *["-e", "--exclude"],
        type=lambda val: re.compile(val),
        help="do not show nodes that match the given regular expression",
    )
    filtering.add_argument(
        *["-o", "--only"],
        type=lambda val: re.compile(val),
        help="only show nodes that match the given regular expression",
    )
