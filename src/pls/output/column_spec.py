from __future__ import annotations

from sys import platform

from pls.globals import state
from pls.models.col_spec import ColumnSpec
from pls.output.detail_columns import detail_columns


column_spec_map: dict[str, ColumnSpec] = {
    "spacer": {"name": " "},  # dummy column to act like spacer
    **detail_columns,
    "icon": {
        # 'icon' is a pseudo-column linked to 'name', so it has no name.
        "name": "",
        "attrs": {"width": 2},
    },
    "name": {"name": "Name" if state.state.no_align else " Name"},
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
        ["type", "perms"],
        ["size"],
        ["ctime", "mtime", "atime"],
    ]
    if platform != "win32":
        col_groups.insert(0, ["inode", "links"])
        col_groups.insert(2, ["user", "group"])
    if state.state.git_root is not None:
        col_groups.append(["git"])
    return col_groups


column_groups = get_column_groups()
"""list of list of column names that are placed together without spacers"""
