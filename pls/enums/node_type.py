from enum import auto
from stat import S_ISBLK, S_ISCHR, S_ISDIR, S_ISFIFO, S_ISLNK, S_ISREG, S_ISSOCK
from typing import Callable

from pls.enums.base import AutoEnum


class NodeType(AutoEnum):
    """
    A node can be either of these:

    - a file
    - a directory
    - a symlink to a different file or directory.

    This enum lists these possibilities.
    """

    FOLDER = auto()  # directory
    FILE = auto()  # regular file
    # TODO: Handle these types of nodes
    CHR = auto()  # character special device file
    BLK = auto()  # block special device file
    FIFO = auto()  # named pipe
    LNK = auto()  # symbolic link
    SOCK = auto()  # socket


type_test_map: dict[NodeType, Callable[[int], bool]] = {
    NodeType.FOLDER: S_ISDIR,
    NodeType.FILE: S_ISREG,
    NodeType.CHR: S_ISCHR,
    NodeType.BLK: S_ISBLK,
    NodeType.FIFO: S_ISFIFO,
    NodeType.LNK: S_ISLNK,
    NodeType.SOCK: S_ISSOCK,
}
"""a mapping of node types with specific functions that evaluate it"""
