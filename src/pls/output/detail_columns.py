from __future__ import annotations

from sys import platform

from pls.models.col_spec import ColumnSpec


detail_columns: dict[str, ColumnSpec] = {
    "inode": {"name": "inode"},
    "links": {"name": "Link#", "attrs": {"justify": "right"}},
    "type": {
        # 'type' is a pseudo-column linked to 'perms', so it has no name.
        "name": ""
    },
    "perms": {"name": "Permissions"},
    "user": {"name": "User"},
    "group": {"name": "Group"},
    "size": {"name": "Size", "attrs": {"justify": "right"}},
    "btime": {"name": "Created at"},
    "ctime": {"name": "Created at" if platform == "win32" else "Changed at"},
    "mtime": {"name": "Modified at"},
    "atime": {"name": "Accessed at"},
    "git": {"name": "Git"},
}
"""columns that can be opted in using the ``--details``/``-d`` flag"""
