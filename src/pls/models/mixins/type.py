from __future__ import annotations

import os
from functools import cached_property
from pathlib import Path
from typing import TYPE_CHECKING, Generic, Optional, TypeVar

from pls.enums.node_type import NodeType, type_char_map, type_test_map
from pls.models.base_node import BaseNode


if TYPE_CHECKING:
    from pls.models.node import Node

    T = TypeVar("T", bound=Node)
else:
    T = TypeVar("T")


class TypeMixin(Generic[T], BaseNode):
    """
    Handles functionality related to the type of the node.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Symlinks

        self.dest_node: Optional[T, str] = None
        self.is_loop: bool = False  # only ``True`` for cyclic symlinks

    @cached_property
    def node_type(self) -> NodeType:
        """whether the node is a file, folder, symlink, FIFO etc."""

        for node_type, node_type_test in type_test_map.items():
            if getattr(self.path, node_type_test)():
                # Symlinks need to set their destination node.
                if node_type == NodeType.SYMLINK and self.dest_node is None:
                    self.populate_dest()
                return node_type
        else:
            return NodeType.UNKNOWN

    @cached_property
    def type_char(self) -> str:
        """the single character representing the file type"""

        return type_char_map[self.node_type]

    def populate_dest(self):
        """
        This sets the dest node for symlinks to a ``Node`` instance pointing to
        the next step in the link. This function ensures that the
        symlink is not unresolvable.
        """

        link_path = os.readlink(self.path)
        try:
            self.path.resolve()  # raises exception if cyclic

            # Use ``os.readlink`` instead of ``Path.resolve`` to step
            # through chained symlinks one-by-one.
            link = Path(link_path)
            if not link.is_absolute():
                link = self.path.parent.joinpath(link)

            self.dest_node = self.__class__(name=link_path, path=link)
        except RuntimeError as exc:
            if "Symlink loop" in str(exc):
                self.is_loop = True
                self.dest_node = link_path
