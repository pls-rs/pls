from __future__ import annotations

from typing import TYPE_CHECKING

from pls.enums.node_type import NodeType
from pls.models.tree import Tree


if TYPE_CHECKING:
    from pls.models.node import Node


class ChildrenComp:
    """
    Adds functionality related to the recursive expansion of subdirectories inside every
    directory.
    """

    def __init__(self, node: Node):
        self.node = node

    def find_children(self):
        """
        Find and link all children nodes of the current node. Also invoke the same
        function on all children nodes to recursively expand them.
        """

        if self.node.type_comp.node_type != NodeType.DIR:
            return

        for child_path in self.node.path.glob("*"):
            child_node = type(self.node)(name=child_path.name, path=child_path)
            Tree.link(self.node, child_node)
            child_node.children_comp.find_children()
