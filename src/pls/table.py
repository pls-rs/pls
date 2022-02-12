from __future__ import annotations

from sys import platform

from rich.console import Console
from rich.table import Table

from pls.args import args
from pls.enums.icon_type import IconType
from pls.models.node import Node


console = Console()
column_spec = {
    "": {"name": ""},  # dummy column to act like spacer
    "type": {
        # 'type' is a pseudo-column linked to 'perms', so it has no name.
        "name": ""
    },
    "perms": {"name": "Permissions"},
    "user": {"name": "User"},
    "group": {"name": "Group"},
    "size": {"name": "Size", "attrs": {"justify": "right"}},
    "icon": {
        # 'icon' is a pseudo-column linked to 'name', so it has no name.
        "name": "",
        "attrs": {"width": 2},
    },
    "name": {
        # The names have a leading space when the leading dots are aligned.
        "name": "Name"
        if args.no_align
        else " Name"
    },
}
"""a mapping of column keys to column spec"""

settings = {
    "padding": (0, 1, 0, 0),
    "box": None,
    "show_header": args.details,
    "header_style": "underline",
}
"""the settings for the Rich table"""


def get_columns() -> list[str]:
    """
    Get the list of columns to show.

    :return: the list of column keys
    """

    cols = []
    if args.details:
        cols.extend(["type", "perms", ""])
        if platform != "win32":
            cols.extend(["user", "group", ""])
        cols.extend(["size", ""])
    if args.icon != IconType.NONE:
        cols.append("icon")
    cols.append("name")
    return cols


def get_table() -> Table:
    """
    Get a Rich table with pre-configured columns. The attributes of the columns
    are retrieved from ``column_spec`` based on keys from ``get_columns``.

    :return: a Rich table
    """

    table = Table(**settings)
    for col_key in get_columns():
        col = column_spec.get(col_key)
        table.add_column(col.get("name"), **col.get("attrs", {}))
    return table


def write_output(all_nodes: list[Node]):
    """
    Write the list of directories and files to the screen as a table.

    :param all_nodes: the list of directories and files
    """

    table = get_table()

    for node in all_nodes:
        data = node.table_row
        if data is not None:
            cells = [data.get(col, "") for col in get_columns()]
            table.add_row(*cells)

    console.print(table)
