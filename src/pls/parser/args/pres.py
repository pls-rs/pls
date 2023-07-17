import argparse

from pls.enums.icon_type import IconType


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for presentation to the given parser.

    :param parser: the parser to which to add the arguments
    """

    presentation = parser.add_argument_group(
        title="presentation",
        description="arguments for controlling the presentation of nodes",
    )
    presentation.add_argument(
        *["-i", "--icon"],
        metavar="TYPE",
        type=IconType,
        choices=list(IconType),
        help="the type of icons to show with the files",
    )
    presentation.add_argument(
        "--align",
        action="boolean_optional",
        help="[underline]do[/]/[magenta]don't[/] align names based on leading dots",
    )
    presentation.add_argument(
        "--multi-cols",
        action="boolean_optional",
        help="render output in multiple/[magenta][underline]single[/][/] columns",
    )
    presentation.add_argument(
        *["-c", "--collapse"],
        action="store_or_count",
        help="collapse autogenerated files behind their sources",
    )
    presentation.add_argument(
        "--tree",
        action="boolean_optional",
        help="do/[magenta][underline]don't[/][/] expand directories recursively",
    )