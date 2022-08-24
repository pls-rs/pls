from __future__ import annotations

from typing import TYPE_CHECKING

from pls.config import constants
from pls.globals import args


if TYPE_CHECKING:
    from typing import Optional


class Tree:
    """
    This class provides functionality associated with rendering trees. It should be
    inherited by any class that behaves as a tree.
    """

    @staticmethod
    def link(parent: Tree, *children: Tree):
        """
        Link a parent node with any number of children nodes.

        :param parent: the parent node to which the children are being linked
        :param children: the children to each of which the parent is being linked
        """

        for child in children:
            child.parent = parent
            parent.children.append(child)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.children: list[Tree] = []
        self.parent: Optional[Tree] = None

        self.pre_shapes: list[str] = []
        self.last_shape: str = ""

    @property
    def is_sub(self) -> bool:  # TODO: Use TypeGuards
        """whether the node is a sub node of another"""

        return self.parent is not None

    @property
    def tree_prefix(self) -> str:
        """the complete string to draw the tree lines before the node name"""

        tree_chars = "".join([*self.pre_shapes, self.last_shape])
        if args.args.align:
            tree_chars = f" {tree_chars}"
        return tree_chars

    def get_shape(self, end_shape: str, not_end_shape: str) -> str:
        """
        Get the correct box-drawing characters based on whether the node is
        the last of its siblings.

        :param end_shape: the box-drawing characters for the last node
        :param not_end_shape: the box-drawing characters for other nodes
        :return: ``end_shape`` if the node is last, ``not_end_shape`` otherwise
        """

        if not self.is_sub:
            return ""
        assert self.parent is not None

        siblings = self.parent.children

        is_last = siblings.index(self) == len(siblings) - 1
        if is_last:
            return end_shape

        return not_end_shape

    def _get_pre_shape(self) -> str:
        """
        Get the box-drawing characters to render before the node's children. The
        pre-shapes are the sets of box drawing characters pertaining to a node's
        ancestors.

        Can either be ``"  "`` or ``"| "``.

        :return: the set of box-drawing characters before the node's own
        """

        return self.get_shape(
            constants.constants.lookup("tree", "space_space", default=""),
            constants.constants.lookup("tree", "pipe_space", default=""),
        )

    def _get_last_shape(self) -> str:
        """
        Get the box-drawing characters to render before the node itself. The
        last shape is the box drawing characters pertaining to itself.

        Can either be ``"└ "`` or ``"├ "``.

        :return: the node's own box-drawing characters
        """

        return self.get_shape(
            constants.constants.lookup("tree", "bend_dash", default=""),
            constants.constants.lookup("tree", "tee_dash", default=""),
        )

    def set_sub_pre_shapes(self):
        """
        Recursively set the pre-shapes of a node on all its descendants.
        """

        pre_shapes = [*self.pre_shapes, self._get_pre_shape()]
        for sub_node in self.children:
            sub_node.pre_shapes.extend(pre_shapes)
            sub_node.set_sub_pre_shapes()
            sub_node.last_shape = sub_node._get_last_shape()
