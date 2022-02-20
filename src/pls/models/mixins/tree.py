from __future__ import annotations

from typing import TYPE_CHECKING, Generic, Optional, TypeVar, cast

from pls.args import args
from pls.data import box_drawing
from pls.models.base_node import BaseNode


if TYPE_CHECKING:
    from pls.models.node import Node

    T = TypeVar("T", bound=Node)
else:
    T = TypeVar("T")


class TreeMixin(Generic[T], BaseNode):
    """
    This mixin provides functionality associated with rendering trees.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.children: list[T] = []
        self.parent: Optional[T] = None

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
        if not args.no_align:
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
            return box_drawing.NONE

        assert self.parent is not None
        siblings = self.parent.children

        is_last = siblings.index(cast(T, self)) == len(siblings) - 1
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

        return self.get_shape(box_drawing.SPACE_SPACE, box_drawing.PIPE_SPACE)

    def _get_last_shape(self) -> str:
        """
        Get the box-drawing characters to render before the node itself. The
        last shape is the box drawing characters pertaining to itself.

        Can either be ``"└ "`` or ``"├ "``.

        :return: the node's own box-drawing characters
        """

        return self.get_shape(box_drawing.BEND_DASH, box_drawing.TEE_DASH)

    def set_sub_pre_shapes(self):
        """
        Recursively set the pre-shapes of a node on all its descendants.
        """

        pre_shapes = [*self.pre_shapes, self._get_pre_shape()]
        for sub_node in self.children:
            sub_node.pre_shapes.extend(pre_shapes)
            sub_node.set_sub_pre_shapes()
            sub_node.last_shape = sub_node._get_last_shape()

    def find_main(self, node_map: dict[str, T]):
        """
        Find the main node of this node from the mapping of node names and
        corresponding ``Node`` instances. If the spec specifies ``collapse`` and
        the main node exists, register this node as a sub-node of the main one.

        :param node_map: the mapping of names and ``Node`` instances
        """

        if collapse := self.spec_attr("collapse"):
            if "extension" in collapse:
                extension = collapse["extension"]
                name = self.name.replace(self.extension, extension)
            else:  # "name" in collapse:
                name = collapse["name"]

            if (node := node_map.get(name)) is not None and node.is_visible:
                node_map[name].children.append(cast(T, self))
                self.parent = node
