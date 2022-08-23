from __future__ import annotations

from enum import auto
from typing import Optional

from pls.config import constants
from pls.enums.base import AutoEnum


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
    BROKEN = auto()  # handling of non-existent nodes


type_test_map: dict[NodeType, str] = {
    node_type: f"is_{node_type.value}"
    for node_type in list(NodeType)
    if node_type not in {NodeType.UNKNOWN, NodeType.BROKEN}
}
"""a mapping of node types with specific functions that evaluate it"""


def get_type_icon(node_type: NodeType) -> Optional[str]:
    """
    Get the icon associated the given node type, which is used when no other specs match
    the node. Generally only the folder type has a default icon.

    :param node_type: the given ``NodeType`` enum item
    :return: the icon associated the given ``NodeType`` value
    """

    return constants.constants.lookup("node_types", node_type.value, "icon")


def get_type_color(node_type: NodeType) -> Optional[str]:
    """
    Get the color associated with the given node type, which is used when no other specs
    match the node. Generally only, the folder type has a default color.

    :param node_type: the given ``NodeType`` enum item
    :return: the color associated with the given ``NodeType`` value
    """

    return constants.constants.lookup("node_types", node_type.value, "color")


def get_type_char(node_type: NodeType) -> str:
    """
    Get the unique, distinct type character associated with the given node type. Returns
    a blank string if no type character is associated.

    :param node_type: the given ``NodeType`` enum item
    :return: the type character mapped to the given ``NodeType`` value
    """

    val = constants.constants.lookup("node_types", node_type.value, "type_char")
    return val if val is not None else ""


def get_type_suffix(node_type: NodeType) -> str:
    """
    Get the unique, distinct type suffix associated with the given node type. Returns
    a blank string if no type character is associated.

    :param node_type: the given ``NodeType`` enum item
    :return: the type suffix mapped to the given ``NodeType`` value
    """

    val = constants.constants.lookup("node_types", node_type.value, "type_suffix")
    return val if val is not None else ""
