from __future__ import annotations

import re
from functools import cached_property
from typing import Optional, Union

from pls.config import icons
from pls.enums.icon_type import IconType
from pls.enums.node_type import NodeType
from pls.globals import args
from pls.models.base_node import BaseNode
from pls.models.mixins.git import GitMixin
from pls.models.mixins.imp import ImpMixin
from pls.models.mixins.stat import StatMixin
from pls.models.mixins.tree import TreeMixin
from pls.models.mixins.type import TypeMixin


class Node(
    GitMixin,
    ImpMixin,
    StatMixin,
    TreeMixin["Node"],
    TypeMixin["Node"],
    BaseNode,
):
    """
    A node is any file, folder or symlink on the file-system. This model stores
    attributes pertaining to a single FS node. Nodes are read from the file
    system directly.
    """

    def __init__(self, is_pseudo: bool = False, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.is_pseudo = is_pseudo  # true for symlink destinations

    def __eq__(self, other: object) -> bool:
        """
        Compare the object ``self`` to the any other object.

        :param other: the node to compare this node with for equality
        :return: ``True`` if the node instances are equal, ``False`` otherwise
        """

        if not isinstance(other, Node):
            # Nodes cannot be compared with any other type.
            return False
        return self.path.resolve() == other.path.resolve()

    def __repr__(self) -> str:
        """
        Get the string representation of the ``Node`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        return f"{self.name} @ {self.path}"

    @property
    def is_visible(self):
        """whether the node deserves to be rendered to the screen"""

        return self.is_visible_imp and self.is_visible_tree

    @cached_property
    def format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup"""

        format_rules = []

        # font color
        if not self.exists:
            format_rules.append("red")  # only happens for broken symlinks
        elif spec_color := self.spec_attr("color"):
            format_rules.append(str(spec_color))
        elif self.node_type == NodeType.DIR:
            format_rules.append("cyan")

        if self.importance_format:
            format_rules.append(self.importance_format)
        if self.git_format:
            format_rules.append(self.git_format)

        if self.name == ".pls.yml":
            format_rules.append("italic")

        if format_rules:
            left = f"[{' '.join(format_rules)}]"
            right = "[/]"
        else:
            left = right = ""
        return left, right

    @cached_property
    def formatted_suffix(self) -> str:
        """the symbol after the filename representing its type"""

        if not self.exists:
            return "⚠"

        if self.node_type == NodeType.SYMLINK:
            assert self.dest_node is not None

            if self.is_loop:
                assert isinstance(self.dest_node, str)
                return f"[dim]@ ↺[/] [red]{self.dest_node}[/red]"

            assert isinstance(self.dest_node, Node)
            return f"[dim]@ →[/] {self.dest_node.formatted_name}"

        mapping = {
            NodeType.DIR: "/",
            NodeType.SOCKET: "=",
            NodeType.FIFO: "|",
        }
        suffix = mapping.get(self.node_type, "")
        if suffix:
            suffix = f"[dim]{suffix}[/]"
        return suffix

    @cached_property
    def formatted_name(self) -> str:
        """the name, formatted using Rich console formatting markup"""

        name = self.pure_name if args.args.align else self.name
        if self.formatted_suffix:
            name = f"{name}{self.formatted_suffix}"

        # Apply format pair.
        left, right = self.format_pair
        name = f"{left}{name}{right}"

        if args.args.align and not self.is_pseudo:
            if re.match(r"\.[^.]", self.name):
                name = f"[dim].[/dim]{name}"
            else:
                # Left pad name with a space to account for leading dots.
                name = f" {name}"

        if self.is_sub:
            name = f"[dim]{self.tree_prefix}[/]{name}"

        return name

    @cached_property
    def formatted_icon(self) -> str:
        """the emoji or Nerd Font icon to show beside the node"""

        if args.args.icon == IconType.NONE:
            return ""

        if args.args.icon == IconType.EMOJI:
            icon_index = icons.emoji_icons
        else:  # args.args.icon == IconType.NERD (default)
            icon_index = icons.nerd_icons

        if spec_icon := self.spec_attr("icon"):
            icon = icon_index.get(str(spec_icon))
        elif self.node_type == NodeType.DIR:
            icon = icon_index.get("folder")
        else:
            icon = None

        if icon:
            # Apply format pair.
            left, right = self.format_pair
            icon = f"{left}{icon}{right}"
        else:
            icon = ""
        return icon

    @cached_property
    def table_row(self) -> Optional[dict[str, Optional[str]]]:
        """the mapping of column names and value when tabulating the node"""

        if not (self.exists and self.is_visible):
            return None
        assert self.stat is not None

        cells: dict[str, Optional[str]] = {
            "name": self.formatted_name,
            "icon": self.formatted_icon,
            "type": self.type_char,
        }

        if not args.args.details:
            return cells  # return early as no more data needed

        cells.update(self.stat_cells)
        cells.update(self.git_cells)

        return cells

    @cached_property
    def sort_keys(self) -> dict[str, Union[str, int, float]]:
        """the mapping of sort fields to their normalised values"""

        keys: dict[str, Union[str, int, float]] = {
            "name": self.canonical_name,
            "ext": self.extension,
            "type": self.type_char,
        }
        keys.update(self.stat_keys)

        return keys
