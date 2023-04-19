from __future__ import annotations

import os
from functools import cached_property
from pathlib import Path
from typing import TYPE_CHECKING, Optional

from rich.markup import escape

from pls.config import constants
from pls.enums import node_type as nt
from pls.enums.node_type import NodeType, SymlinkState
from pls.globals import args
from pls.models.format_rules import FormatRules


if TYPE_CHECKING:
    from typing import Union

    from pls.models.node import Node


class TypeComp:
    """
    Adds functionality related to the type of the node. This composition depends on the
    ``StatComp`` being already setup on the ``Node`` instance passed to it.
    """

    def __init__(self, node: Node):
        self.node = node

        self.node_type = NodeType.UNKNOWN
        if node.stat_comp.stat is not None:  # Should always be ``True``.
            try:
                for node_type, node_type_test in nt.type_test_map.items():
                    if node_type_test(node.stat_comp.stat.st_mode):
                        # Symlinks need to set their destination node.
                        self.node_type = node_type
                        break
            except OSError:
                # stat ``S_IS*`` functions can raise errors
                pass

        self.type_char: Optional[str] = self.node_type.get_constant(
            "type_char", default=None
        )
        self.suffix_char: Optional[str] = self.node_type.get_constant(
            "type_suffix", default=None
        )
        self.icon: Optional[str] = self.node_type.get_constant("icon", default=None)

        self.dest_node: Union[str, Node, None] = None
        self.symlink_state = SymlinkState.OK  # not relevant unless node is a symlink

        if self.node_type == NodeType.SYMLINK:
            self.populate_dest()

    @cached_property
    def cells(self) -> dict[str, str]:
        """mapping of details keys to the information pertaining to node type"""

        return {
            # Uses ``format_icon`` because the type char is more like an icon in nature.
            "type": self.node.format_rules.format_icon(self.type_char)
            if self.type_char is not None
            else "",
        }

    @cached_property
    def format_rules(self) -> FormatRules:
        """the formatting associated with a node's type"""

        fmt_rules = FormatRules()

        if styles := self.node_type.get_constant("color", default=None):
            fmt_rules.append(styles)

        return fmt_rules

    @cached_property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        if self.node_type == NodeType.UNKNOWN:
            is_type_visible = True
        elif self.node_type == NodeType.DIR:
            is_type_visible = args.args.dirs
        else:
            is_type_visible = args.args.files
        return is_type_visible

    @cached_property
    def keys(self) -> dict[str, Union[str, int, float]]:
        """mapping of detail keys to the corresponding raw stat results"""

        return {
            "cat": "dir" if self.node_type == NodeType.DIR else "file",
            "type": self.type_char if self.type_char is not None else "",
        }

    @cached_property
    def display_suffix(self) -> str:
        """the text after the filename representing its type"""

        suffix = self.suffix_char if self.suffix_char is not None else ""
        if self.node_type == NodeType.SYMLINK:
            if self.symlink_state == SymlinkState.LOOP:
                assert isinstance(self.dest_node, str)
                icon = constants.constants.lookup(
                    "pointers", "symlink_loop", default="↺"
                )
                suffix = f"[dim]{suffix} {icon}[/] [red]{self.dest_node}[/]"
            elif self.symlink_state == SymlinkState.BROKEN:
                assert isinstance(self.dest_node, str)
                icon = constants.constants.lookup(
                    "pointers", "symlink_broken", default="↝"
                )
                suffix = f"[dim]{suffix} {icon}[/] [red]{self.dest_node}⚠[/]"
            else:  # self.symlink_state == SymlinkState.OK
                if TYPE_CHECKING:  # ``Node`` is only imported when ``TYPE_CHECKING``.
                    assert isinstance(self.dest_node, Node)
                icon = constants.constants.lookup(
                    "pointer", "symlink_dest", default="→"
                )
                suffix = f"[dim]{suffix} {icon}[/] {self.dest_node.formatted_name}"
        elif suffix:
            suffix = f"[dim]{suffix}[/]"
        return suffix

    def populate_dest(self):
        """
        This sets the dest node for symlinks to a ``Node`` instance pointing to the next
        step in the link. This function ensures that the symlink is not unresolvable.
        """

        link_path = os.readlink(self.node.path)
        try:
            self.node.path.resolve()  # raises exception if cyclic

            # Use ``os.readlink`` instead of ``Path.resolve`` to step through the
            # chained symlinks one-by-one.
            link = Path(link_path)
            if not link.is_absolute():
                link = self.node.path.parent.joinpath(link)

            # ``dest_node`` is type ``Node``.
            link.lstat()
            self.dest_node = type(self.node)(name=link_path, path=link, is_pseudo=True)
        except (OSError, RuntimeError) as exc:
            if isinstance(exc, RuntimeError) and "Symlink loop" in str(exc):
                self.symlink_state = SymlinkState.LOOP
                # ``dest_node`` is type ``str``.
                self.dest_node = escape(link_path)
            else:
                self.symlink_state = SymlinkState.BROKEN
                # ``dest_node`` is type ``str``.
                self.dest_node = escape(link_path)
