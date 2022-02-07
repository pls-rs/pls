from __future__ import annotations

import os
from functools import cached_property
from pathlib import Path
from typing import Union

from pls.args import args
from pls.data.getters import emoji_icons, nerd_icons
from pls.enums.icon_type import IconType
from pls.enums.node_type import NodeType
from pls.fs.stats import (
    get_group,
    get_node_type,
    get_permission_text,
    get_size,
    get_user,
)
from pls.models.node_spec import NodeSpec


class Node:
    """
    A node is any file, folder or symlink on the file-system. This model stores
    attributes pertaining to a single FS node.

    Nodes are read from the file system directly using ``os.walk``.
    """

    def __init__(self, name: str, path: Path):
        self.name = name
        self.path = path

        self.stat: Union[os.stat_result, None] = None
        try:
            self.stat = path.lstat()
            self.exists = True
        except FileNotFoundError:
            self.exists = False

        self.dest_node: Union[Node, None] = None  # only populated for symlinks
        self.specs: list[NodeSpec] = []  # matched later (see ``map_specs``)

    def __repr__(self) -> str:
        """
        Get the string representation of the ``Node`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        return f"{self.name} @ {self.path}"

    @cached_property
    def full_name(self) -> str:
        """the name of the node with the appropriate suffix"""

        name = self.name
        if self.suffix:
            name = f"{name}{self.suffix}"

        return name

    @cached_property
    def node_type(self) -> NodeType:
        """whether the node is a file, folder, symlink, FIFO etc."""

        node_type = get_node_type(self.path)

        # Symlinks need to set their destination node
        if node_type == NodeType.SYMLINK and self.dest_node is None:
            # Using this instead of ``Path.resolve`` to be able to step through
            # chained symlinks one by one
            link_path = os.readlink(self.path)
            link = Path(link_path)
            if not link.is_absolute():
                link = self.path.parent.joinpath(link)

            self.dest_node = Node(name=link_path, path=link)

        return node_type

    @cached_property
    def suffix(self) -> str:
        """the symbol after the filename representing its type"""

        if self.node_type == NodeType.SYMLINK:
            dest: Node = self.dest_node
            name = dest.full_name if dest.exists else f"⚠ {dest.name}"
            left, right = dest.format_pair
            return f"[dim]@ →[/] {left}{name}{right}"

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
    def type_char(self) -> str:
        """the single character representing the file type"""

        mapping = {
            NodeType.DIR: "[cyan]d[/]",
            NodeType.SYMLINK: "l",
            NodeType.SOCKET: "s",
            NodeType.FIFO: "p",
            NodeType.BLOCK_DEVICE: "b",
            NodeType.CHAR_DEVICE: "c",
        }
        return mapping.get(self.node_type, "")

    @cached_property
    def ext(self) -> Union[str, None]:
        """the extension of the node, i.e. the portion after the last dot"""

        return self.name.split(".")[-1] if "." in self.name else None

    @cached_property
    def icon(self) -> str:
        """the emoji or Nerd Font icon to show beside the node"""

        if args.icon == IconType.EMOJI:
            icon_index = emoji_icons
        elif args.icon == IconType.NERD:
            icon_index = nerd_icons
        else:  # args.icon == IconType.NONE:
            raise NotImplementedError("Icon should not be needed.")

        if spec_icon := self.spec_attr("icon"):
            icon = icon_index.get(spec_icon)
        elif self.node_type == NodeType.DIR:
            icon = icon_index.get("folder")
        else:
            icon = None
        return icon or ""

    @cached_property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        # If explicitly requested for all files, show all
        if args.all:
            return True

        # Nodes without spec and with a leading dot are hidden
        if not self.specs and self.name.startswith("."):
            return False

        # Nodes with importance -2 are hidden
        if self.spec_attr("importance") == -2:
            return False

        return True

    @cached_property
    def formatted_name(self) -> str:
        """the name, formatted using Rich console formatting markup"""

        name = self.full_name
        if not args.no_align:
            if name.startswith("."):
                name = name.replace(".", "[dim].[/dim]", 1)
            else:
                name = f" {name}"
        return name

    @cached_property
    def format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup"""

        format_rules = []

        # Font color
        if not self.exists:
            format_rules.append("red")  # only happens for broken symlinks
        elif spec_color := self.spec_attr("color"):
            format_rules.append(spec_color)
        elif self.node_type == NodeType.DIR:
            format_rules.append("cyan")

        # Font weight
        if spec_importance := self.spec_attr("importance"):
            if spec_importance == 2:
                format_rules.append("underline")
            elif spec_importance == 1:
                format_rules.append("bold")
            elif spec_importance == -1:
                format_rules.append("dim")

        # Italics
        if self.name == ".pls.yml":
            format_rules.append("italic")

        if format_rules:
            left = f"[{' '.join(format_rules)}]"
            right = "[/]"
        else:
            left = right = ""
        return left, right

    @cached_property
    def table_row(self) -> dict[str, str]:
        """the mapping of column names and value when tabulating the node"""

        left, right = self.format_pair

        cells = dict()
        cells["name"] = f"{left}{self.formatted_name}{right}"
        if args.icon != IconType.NONE:
            cells["icon"] = f"{left}{self.icon}{right}"
        if args.details:
            cells["type"] = self.type_char
            cells["perms"] = get_permission_text(self.stat.st_mode)
            cells["user"] = get_user(self.stat.st_uid)
            cells["group"] = get_group(self.stat.st_gid)
            if self.node_type != NodeType.DIR:
                cells["size"] = get_size(self.stat.st_size)

        return cells

    def spec_attr(self, attr: str) -> Union[str, int, None]:
        """
        Get the requested attribute from the first matching spec to provide it.

        :param attr: the requested attribute
        :return: the value of the attribute if found, ``None`` otherwise
        """

        for spec in self.specs:
            if attr_val := getattr(spec, attr, None):
                return attr_val
        return None

    def match(self, specs: list[NodeSpec]):
        """
        Find all spec matching this node from a list of all possible specs and
        store them in the ``specs`` attribute.

        :param specs: the list of all specs
        """

        self.specs = [spec for spec in specs if spec.match(self.name)]
