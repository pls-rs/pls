import argparse

from pls.enums.unit_system import UnitSystem


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for customising the detailed view to the given parser.

    :param parser: the parser to which to add the arguments
    """

    info_mod = parser.add_argument_group(
        title="info modification",
        description="arguments for modifying the presentation of information",
    )
    info_mod.add_argument(
        *["-u", "--units"],
        type=UnitSystem,
        choices=list(UnitSystem),
        help="the units to use when listing the size of files",
    )
    info_mod.add_argument(
        *["-t", "--time-fmt"],
        help="the template for formatting the timestamps on the file",
    )
