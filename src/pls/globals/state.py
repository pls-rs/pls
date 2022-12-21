from __future__ import annotations

import logging
import os
from pathlib import Path
from typing import Optional

from pls.fs.git import get_git_root, get_git_statuses

logger = logging.getLogger(__name__)


class Singleton(type):
    """
    This metaclass can be inherited by any class to implement the singleton
    pattern. A singleton can only be instantiated one and all subsequent
    initialisations will always return the same instance.
    """

    _instances: dict = {}

    def __call__(cls, *args, **kwargs):
        # Calling a class as ``Klass()`` instantiates it.

        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class State(metaclass=Singleton):
    """
    Holds the global state of the application.
    """

    def __init__(self):
        # See ``setup_user_conf``.
        self.user_conf_dir: Optional[Path] = None

        # See ``setup_home``.
        self.home_dir: Optional[Path] = None

        # See ``setup_user_groups``.
        self.uid: Optional[int] = None
        self.gids: set[int] = set()

        # See ``setup_git``.
        self.git_root: Optional[Path] = None
        self.git_status_map: dict[Path, str] = {}

    def __repr__(self) -> str:
        """
        Get the string representation of the ``State`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        return str(self.__dict__)

    def setup_user_conf(self):
        """
        Set up the user-level config directory.
        """

        try:
            if user_conf_dir := os.getenv("PLS_USER_CONF_DIR"):
                path = Path(user_conf_dir)
                if "~" in user_conf_dir:
                    path = path.expanduser()
                self.user_conf_dir = path.resolve()
                logger.info(f"User config dir: {self.user_conf_dir}")
        except RuntimeError:
            logger.info("User config directory could not be determined.")

    def setup_home(self):
        """
        Set up the home directory of the current user.
        """

        try:
            self.home_dir = Path.home()
            logger.info(f"Home directory: {self.home_dir}")
        except RuntimeError:
            logger.info("Home directory could not be determined.")

    def setup_user_groups(self):
        """
        Set up the username and groups of the current active user.
        """

        try:
            from grp import getgrall
            from pwd import getpwuid
        except ModuleNotFoundError:
            # This happens on non-POSIX systems like Windows.
            return

        self.uid = os.getuid()

        self.gids = set()
        try:
            user = getpwuid(self.uid)
        except KeyError:
            return
        self.gids.add(user.pw_gid)
        username = user.pw_name
        self.gids.update(
            group.gr_gid for group in getgrall() if username in group.gr_mem
        )

    def setup_git(self, node: Path):
        """
        Set up the Git root of the directory whose contents are being listed.

        :param node: the file or directory being listed
        """

        directory = node if node.is_dir() else node.parent
        self.git_root = get_git_root(directory)
        if self.git_root is not None:
            self.git_status_map = get_git_statuses(self.git_root)


state: State
"""the global state of the application"""
