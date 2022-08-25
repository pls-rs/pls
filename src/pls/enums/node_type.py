from __future__ import annotations

import stat
from enum import auto
from typing import Callable

from pls.config import constants
from pls.enums.base import AutoEnum


class SymlinkState(AutoEnum):
    """
    A symlink can be in any of these states:

    - OK, symlink resolves to an existing node
    - broken, symlink resolves to a non-existent node
    - loop, symlink resolves to a chain that eventually leads back to it
    """

    OK = auto()
    BROKEN = auto()
    LOOP = auto()


class NodeType(AutoEnum):
    """
    This enum lists the different types of nodes. Refer to `the Wikipedia article on
    Unix file types <https://en.wikipedia.org/wiki/Unix_file_types>`_ for more info.
    """

    SYMLINK = auto()  # symbolic link, should always be first
    DIR = auto()  # directory
    FIFO = auto()  # named FIFO pipe
    SOCKET = auto()  # file-based socket
    CHAR_DEVICE = auto()  # character special device file
    BLOCK_DEVICE = auto()  # block special device file
    FILE = auto()  # regular file, should always be last
    UNKNOWN = auto()  # graceful handling of unrecognised type

    def get_constant(self, attribute: str, **kwargs) -> str:
        """
        Get the value of a constant attribute associated the given node type.

        :param attribute: the key under the node type to retrieve from the constants
        :return: the icon associated the given ``NodeType`` value
        """

        return constants.constants.lookup("node_types", self.value, attribute, **kwargs)


type_test_map: dict[NodeType, Callable[[int], bool]] = {
    NodeType.SYMLINK: stat.S_ISLNK,
    NodeType.DIR: stat.S_ISDIR,
    NodeType.FIFO: stat.S_ISFIFO,
    NodeType.SOCKET: stat.S_ISSOCK,
    NodeType.CHAR_DEVICE: stat.S_ISCHR,
    NodeType.BLOCK_DEVICE: stat.S_ISBLK,
    NodeType.FILE: stat.S_ISREG,
}
"""a mapping of node types with specific ``stat`` functions that check for them"""
