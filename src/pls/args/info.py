import argparse

from pls.enums.unit_system import UnitSystem
from pls.output.detail_columns import detail_columns


detail_choices = list(detail_columns.keys()) + [
    "def",  # refers to the default set
    "+",  # means all details
]


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for showing and customising details to the given parser.

    :param parser: the parser to which to add the arguments
    """

    info = parser.add_argument_group(
        title="info",
        description="arguments for toggling and customising details for nodes",
    )
    info.add_argument(
        *["-d", "--details"],
        metavar="FIELD",
        action="append",
        nargs=argparse.OPTIONAL,
        help="the data points to show for each node in the output",
        const="def",  # when there is a --details flag without value
        choices=detail_choices,
    )
    info.add_argument(
        *["-u", "--units"],
        metavar="SYSTEM",
        type=UnitSystem,
        choices=list(UnitSystem),
        help="the units to use when listing the size of files",
    )
    info.add_argument(
        *["-t", "--time-fmt"],
        metavar="FMT",
        help="the template for formatting the timestamps on the file",
    )
