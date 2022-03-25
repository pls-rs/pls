from __future__ import annotations

from enum import auto

from pls.config import constants
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
    file types <https://en.wikipedia.org/wiki/Unix_file_types>`_ for more info.
    """

    SYMLINK = auto()  # symbolic link
    DIR = auto()  # directory
    FILE = auto()  # regular file
    FIFO = auto()  # named pipe
    SOCKET = auto()  # socket
    CHAR_DEVICE = auto()  # character special device file
    BLOCK_DEVICE = auto()  # block special device file
    UNKNOWN = auto()  # graceful handling of unrecognised type


type_test_map: dict[NodeType, str] = {
    node_type: f"is_{node_type.value}"
    for node_type in list(NodeType)
    if node_type != NodeType.UNKNOWN
}
"""a mapping of node types with specific functions that evaluate it"""


def get_type_char_map() -> dict[NodeType, str]:
    """
    Map each node type with its unique distinct type character.

    :return: the mapping of ``NodeType`` values to type characters.
    """

    mapping: dict[NodeType, str] = {}
    for node_type in list(NodeType):
        mapping[node_type] = constants.constants.get("type_chars", {}).get(
            node_type.value, " "
        )
    return mapping


type_char_map: dict[NodeType, str] = get_type_char_map()
