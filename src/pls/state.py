from __future__ import annotations

import os
from functools import cached_property
from pathlib import Path

from pls.args import args
from pls.fs.git import get_git_root, get_git_statuses


class State:
    """
    Holds the global state of the application.
    """

    def __init__(self):
        self.git_root = get_git_root(args.directory)
        self.git_status_map: dict[Path, str] = dict()
        if self.is_git_managed:
            self.git_status_map = get_git_statuses(self.git_root)

        try:
            from grp import getgrall, getgrgid
            from pwd import getpwnam, getpwuid

            self.username = getpwuid(os.getuid()).pw_name
            self.groups = set(
                group.gr_name for group in getgrall() if self.username in group.gr_mem
            )
            gid = getpwnam(self.username).pw_gid
            self.groups.add(getgrgid(gid).gr_name)
        except ModuleNotFoundError:  # on non-POSIX systems like Windows
            self.username = None
            self.groups = set()

    def __repr__(self) -> str:
        """
        Get the string representation of the ``State`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        return str(self.__dict__)

    @cached_property
    def is_git_managed(self) -> bool:
        """whether the working directory is managed by Git"""

        return self.git_root is not None


state = State()
"""the global state of the application"""
