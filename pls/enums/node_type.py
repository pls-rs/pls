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

    FILE = auto()
    FOLDER = auto()
    SYMLINK = auto()  # TODO: Handle these types of nodes
