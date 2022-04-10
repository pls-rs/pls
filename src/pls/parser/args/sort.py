import argparse

from pls.output.columns.detail_columns import detail_column_specs


sort_choices = ["cat", "name", "ext"]

# Allow sorting by certain details
invalid_keys = {"perms", "git"}
sort_choices += [key for key in detail_column_specs.keys() if key not in invalid_keys]

# Add a hyphen-suffixed version for reversed sorting
sort_choices += [f"{key}-" for key in sort_choices]


def add_args(parser: argparse.ArgumentParser):
    """
    Add arguments for sorting to the given parser.

    :param parser: the parser to which to add the arguments
    """

    sorting = parser.add_argument_group(
        title="sorting",
        description="arguments used for sorting nodes in the output",
    )
    sorting.add_argument(
        *["-s", "--sort"],
        metavar="KEY",
        action="append",
        help="the field based on which to sort the files and directories",
        choices=sort_choices,
    )
