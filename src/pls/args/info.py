import argparse

from pls.output.detail_columns import detail_columns


detail_choices = list(detail_columns.keys()) + [
    "def",  # refers to the default set
    "+",  # means all details
]


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for the detailed view to the given parser.

    :param parser: the parser to which to add the arguments
    """

    info = parser.add_argument_group(
        title="info",
        description="arguments for controlling the amount of info for nodes",
    )
    info.add_argument(
        *["-d", "--details"],
        action="append",
        nargs=argparse.OPTIONAL,
        help="the data points to show for each node in the output",
        const="def",  # when there is a --details flag without value
        choices=detail_choices,
    )
