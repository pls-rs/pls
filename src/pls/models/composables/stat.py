from __future__ import annotations

import os
from functools import cached_property
from typing import TYPE_CHECKING

from pls.fs.stats import (
    get_formatted_group,
    get_formatted_links,
    get_formatted_perms,
    get_formatted_size,
    get_formatted_time,
    get_formatted_user,
)
from pls.globals import args


if TYPE_CHECKING:
    from typing import Callable, Optional, Union

    from pls.models.node import Node


class StatComp:
    """
    Adds functionality related to the node's ``stat`` information such as ownership,
    permissions and size.
    """

    def __init__(self, node: Node):
        self.node = node

        self.stat: Optional[os.stat_result] = None
        try:
            self.stat = node.path.lstat()  # does not follow symlinks
        except (FileNotFoundError, PermissionError):
            pass

    @cached_property
    def cells(self) -> dict[str, str]:
        """mapping of detail keys to the corresponding formatted stat results"""

        if self.stat is None:
            return {}

        column_function_map: dict[str, tuple[Callable, tuple]] = {
            "inode": (str, (self.stat.st_ino,)),
            "links": (
                get_formatted_links,
                (self.node.type_comp.node_type, self.stat.st_nlink),
            ),
            "perms": (get_formatted_perms, (self.stat.st_mode,)),
            "user": (get_formatted_user, (self.stat.st_uid,)),
            "group": (get_formatted_group, (self.stat.st_gid,)),
            "size": (
                get_formatted_size,
                (self.node.type_comp.node_type, self.stat.st_size),
            ),
            "btime": (get_formatted_time, (getattr(self.stat, "st_birthtime", None),)),
            "ctime": (get_formatted_time, (getattr(self.stat, "st_ctime", None),)),
            "mtime": (get_formatted_time, (getattr(self.stat, "st_mtime", None),)),
            "atime": (get_formatted_time, (getattr(self.stat, "st_atime", None),)),
        }

        cells = {
            column: function(*(func_args or (self.stat,)))
            for column, (function, func_args) in column_function_map.items()
            if column in args.args.details
        }

        return cells

    @cached_property
    def keys(self) -> dict[str, Union[str, int, float]]:
        """mapping of detail keys to the corresponding raw stat results"""

        if self.stat is None:
            return {}

        keys: dict[str, Union[str, int, float]] = {
            "inode": self.stat.st_ino,
            "links": self.stat.st_nlink,
            "size": self.stat.st_size,
            "btime": getattr(self.stat, "st_birthtime", 0),
            "ctime": self.stat.st_ctime,
            "mtime": self.stat.st_mtime,
            "atime": self.stat.st_atime,
        }
        return keys
