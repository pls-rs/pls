from __future__ import annotations

from enum import auto

from pls.enums.base import AutoEnum


class NodeType(AutoEnum):
    """
    A node can be either of these:

    - a file
    - a directory
    - a symlink to a different file or directory.

    This enum lists these possibilities.
    """

    DIR = auto()  # directory
    FILE = auto()  # regular file
    FIFO = auto()  # named pipe
    CHAR_DEVICE = auto()  # character special device file
    BLOCK_DEVICE = auto()  # block special device file
    SYMLINK = auto()  # symbolic link
    SOCKET = auto()  # socket


type_test_map: dict[NodeType, str] = {
    node_type: f"is_{node_type.value}" for node_type in list(NodeType)
}
"""a mapping of node types with specific functions that evaluate it"""
