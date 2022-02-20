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
from pls.models.base_node import BaseNode


if TYPE_CHECKING:
    from typing import Callable, Optional


class StatMixin(BaseNode):
    """
    Handles functionality related to OS stat results.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.stat: Optional[os.stat_result] = None
        try:
            self.stat = self.path.lstat()
        except FileNotFoundError:
            pass

    @cached_property
    def exists(self) -> bool:  # TODO: Use TypeGuards
        """whether the given instance exists on the underlying file system"""

        return self.stat is not None

    @cached_property
    def stat_cells(self) -> dict[str, str]:
        """mapping of detail keys to the corresponding formatted stat results"""

        if not self.exists:
            return {}

        assert self.stat is not None
        column_function_map: dict[str, tuple[Callable, tuple]] = {
            "links": (get_formatted_links, ()),
            "perms": (get_formatted_perms, ()),
            "user": (get_formatted_user, ()),
            "group": (get_formatted_group, ()),
            "size": (get_formatted_size, ()),
            "ctime": (get_formatted_time, ("st_ctime",)),
            "mtime": (get_formatted_time, ("st_mtime",)),
            "atime": (get_formatted_time, ("st_atime",)),
        }

        cells = {"inode": str(self.stat.st_ino)}
        for column, (function, func_args) in column_function_map.items():
            cells[column] = function(self.stat, *func_args)

        return cells

    @cached_property
    def stat_keys(self) -> dict:
        """mapping of detail keys to the corresponding raw stat results"""

        if not self.exists:
            return {}

        assert self.stat is not None
        return {
            "inode": self.stat.st_ino,
            "links": self.stat.st_nlink,
            "size": self.stat.st_size,
            "ctime": self.stat.st_ctime,
            "mtime": self.stat.st_mtime,
            "atime": self.stat.st_atime,
        }
