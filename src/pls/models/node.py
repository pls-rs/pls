from __future__ import annotations

import re
from functools import cached_property
from typing import Optional, Union

from pls.config import icons
from pls.enums.icon_type import IconType
from pls.enums.node_type import NodeType, get_type_suffix
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

    @cached_property
    def format_rules(self) -> tuple[list[str], list[str]]:
        """the list of formatting rules to apply to the icons and text respectively"""

        fmt_rules = []
        name_fmt_rules = []

        # font color
        if not self.exists:
            fmt_rules.append("red")  # only happens for broken symlinks
        elif spec_color := self.spec_attr("color"):
            fmt_rules.append(str(spec_color))
        elif self.node_type == NodeType.DIR:
            fmt_rules.append("cyan")

        imp_fmt_rules, imp_txt_fmt_rules = self.importance_format_rules
        if imp_fmt_rules:
            fmt_rules.extend(imp_fmt_rules)
        if imp_txt_fmt_rules:
            name_fmt_rules.extend(imp_txt_fmt_rules)

        git_fmt_rules, git_txt_fmt_rules = self.git_format_rules
        if git_fmt_rules:
            fmt_rules.extend(git_fmt_rules)
        if git_txt_fmt_rules:
            name_fmt_rules.extend(git_txt_fmt_rules)

        if self.name == ".pls.yml":
            name_fmt_rules.append("italic")

        name_fmt_rules.extend(fmt_rules)
        return fmt_rules, name_fmt_rules

    @staticmethod
    def _get_format_pair(format_rules: list[str]) -> tuple[str, str]:
        """
        Get a two element tuple containing the opening and closing tags of Rich console
        formatting markup.

        :param format_rules: the rules to convert to console markup
        :return: the pair of opening and closing formatting tags
        """

        if format_rules:
            left = f"[{' '.join(format_rules)}]"
            right = "[/]"
        else:
            left = right = ""
        return left, right

    @cached_property
    def name_format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup for text"""

        _, name_fmt_rules = self.format_rules
        return self._get_format_pair(name_fmt_rules)

    @cached_property
    def format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup for icons"""

        fmt_rules, _ = self.format_rules
        return self._get_format_pair(fmt_rules)

    @cached_property
    def formatted_suffix(self) -> str:
        """the symbol after the filename representing its type"""

        node_type = self.node_type if self.exists else NodeType.BROKEN
        suffix = get_type_suffix(node_type)
        if self.node_type == NodeType.SYMLINK:
            if suffix:
                suffix = f"{suffix} "
            assert self.dest_node is not None

            if self.is_loop:
                assert isinstance(self.dest_node, str)
                return f"[dim]{suffix}↺[/] [red]{self.dest_node}[/red]"

            assert isinstance(self.dest_node, Node)
            return f"[dim]{suffix}→[/] {self.dest_node.formatted_name}"

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
        left, right = self.name_format_pair
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
    def formatted_type_char(self) -> str:
        """the type character associated with the type of the node"""

        left, right = self.format_pair
        return f"{left}{self.type_char}{right}"

    @cached_property
    def table_row(self) -> Optional[dict[str, Optional[str]]]:
        """the mapping of column names and value when tabulating the node"""

        if not (self.exists and self.is_visible):
            return None
        assert self.stat is not None

        cells: dict[str, Optional[str]] = {
            "name": self.formatted_name,
            "icon": self.formatted_icon,
            "type": self.formatted_type_char,
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
            "cat": "dir" if self.node_type == NodeType.DIR else "file",
            "name": self.canonical_name,
            "ext": self.extension,
            "type": self.type_char,
        }
        keys.update(self.stat_keys)

        return keys
