from __future__ import annotations

from sys import platform

from pls.globals import state
from pls.models.column_spec import ColumnSpec


detail_column_specs: dict[str, ColumnSpec] = {
    "inode": ColumnSpec(
        key="inode",
        name="inode",
        is_available=platform != "win32",
    ),
    "links": ColumnSpec(
        key="links",
        name="Link#",
        attrs={"justify": "right"},
        is_available=platform != "win32",
    ),
    "type": ColumnSpec(key="type", name=""),  # pseudo-column
    "perms": ColumnSpec(key="perms", name="Permissions"),
    "user": ColumnSpec(
        key="user",
        name="User",
        is_available=platform != "win32",
    ),
    "group": ColumnSpec(
        key="group",
        name="Group",
        is_available=platform != "win32",
    ),
    "size": ColumnSpec(key="size", name="Size"),
    "btime": ColumnSpec(
        key="btime",
        name="Created at",
        is_available=(platform == "darwin"),
    ),
    "ctime": ColumnSpec(
        key="ctime",
        name="Created at" if platform == "win32" else "Changed at",
    ),
    "mtime": ColumnSpec(key="mtime", name="Modified at"),
    "atime": ColumnSpec(key="atime", name="Accessed at"),
    "git": ColumnSpec(
        key="git",
        name="Git",
        is_available=(lambda: state.state.git_root is not None),
    ),
}
"""a mapping of detail column keys to column specs"""

detail_column_groups: list[list[str]] = [
    ["inode", "links"],
    ["type", "perms"],
    ["user", "group"],
    ["size"],
    ["btime", "ctime", "mtime", "atime"],
    ["git"],
]
"""list of list of column names that are placed together without spacers"""
