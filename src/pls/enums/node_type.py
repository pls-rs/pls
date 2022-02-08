from __future__ import annotations

from enum import auto

from pls.enums.base import AutoEnum


class NodeType(AutoEnum):
    """
    A node can be either of these:

    - a directory
    - a regular file
    - a name FIFO pipe
    - a file-based socket
    - a char device
    - a block device
    - a symlink to any of the above

    This enum lists these possibilities. Refer to `the Wikipedia article on Unix
    file types`_ for more info.

    .. _: https://en.wikipedia.org/wiki/Unix_file_types
    """

    SYMLINK = auto()  # symbolic link
    DIR = auto()  # directory
    FILE = auto()  # regular file
    FIFO = auto()  # named pipe
    SOCKET = auto()  # socket
    CHAR_DEVICE = auto()  # character special device file
    BLOCK_DEVICE = auto()  # block special device file


type_test_map: dict[NodeType, str] = {
    node_type: f"is_{node_type.value}" for node_type in list(NodeType)
}
"""a mapping of node types with specific functions that evaluate it"""
