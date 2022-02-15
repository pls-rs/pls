from __future__ import annotations

from sys import platform

from pls.args import args
from pls.models.col_spec import ColumnSpec
from pls.state import state
from pls.table.detail_columns import detail_columns


column_spec_map: dict[str, ColumnSpec] = {
    "spacer": {"name": " "},  # dummy column to act like spacer
    **detail_columns,
    "icon": {
        # 'icon' is a pseudo-column linked to 'name', so it has no name.
        "name": "",
        "attrs": {"width": 2},
    },
    "name": {"name": "Name" if args.no_align else " Name"},
}
"""a mapping of column keys to column spec"""


def get_column_groups() -> list[list[str]]:
    """
    Get the grouping of columns which determines the position of spacers.
    Spacer columns are inserted after every group (except the last) that has at
    least one visible member.

    :return: the standard column groups
    """

    col_groups = [
        ["inode", "links"],
        ["type", "perms"],
        ["size"],
        ["ctime", "mtime", "atime"],
    ]
    if platform != "win32":
        col_groups.insert(2, ["user", "group"])
    if state.is_git_managed:
        col_groups.append(["git"])
    return col_groups


column_groups = get_column_groups()
"""list of list of column names that are placed together without spacers"""
