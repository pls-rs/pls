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
from pls.state import State


class Node:
    """
    A node is any file, folder or symlink on the file-system. This model stores
    attributes pertaining to a single FS node.

    Nodes are read from the file system directly using ``os.walk``.
    """

    def __init__(self, name: str, path: Path, state: State):
        self.name = name
        self.path = path

        self.state = state  # keeping a copy to pass to dest_nodes
        self.is_git_managed = state.is_git_managed
        if self.is_git_managed:
            self.path_wrt_git = path.relative_to(state.git_root)
            self.git_status = state.git_status_map.get(self.path_wrt_git, "  ")
        else:
            self.path_wrt_git = None
            self.git_status = None

        self.stat: Union[os.stat_result, None] = None
        try:
            self.stat = path.lstat()
            self.exists = True
        except FileNotFoundError:
            self.exists = False

        self.is_loop: bool = False  # only ``True`` for cyclic symlinks
        self.dest_node: Union[Node, str, None] = None  # only populated for symlinks

        self.specs: list[NodeSpec] = []  # matched later (see ``map_specs``)

    def __eq__(self, other: Node) -> bool:
        """
        Compare the object ``self`` to the other instance of ``Node``.

        :param other: the node to compare this node with for equality
        :return: ``True`` if the node instances are equal, ``False`` otherwise
        """

        return self.path.resolve() == other.path.resolve()

    def __repr__(self) -> str:
        """
        Get the string representation of the ``Node`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        return f"{self.name} @ {self.path}"

    @cached_property
    def node_type(self) -> NodeType:
        """whether the node is a file, folder, symlink, FIFO etc."""

        node_type = get_node_type(self.path)

        # Symlinks need to set their destination node.
        if node_type == NodeType.SYMLINK and self.dest_node is None:
            link_path = os.readlink(self.path)
            try:
                self.path.resolve()  # raises exception if cyclic

                # Use ``os.readlink`` instead of ``Path.resolve`` to step
                # through chained symlinks one-by-one.
                link = Path(link_path)
                if not link.is_absolute():
                    link = self.path.parent.joinpath(link)

                self.dest_node = Node(name=link_path, path=link, state=self.state)
            except RuntimeError as exc:
                if "Symlink loop" in str(exc):
                    self.is_loop = True
                    self.dest_node = link_path

        return node_type

    @cached_property
    def ext(self) -> Union[str, None]:
        """the extension of the node, i.e. the portion after the last dot"""

        return self.name.split(".")[-1] if "." in self.name else None

    @cached_property
    def format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup"""

        format_rules = []

        # font color
        if not self.exists:
            format_rules.append("red")  # only happens for broken symlinks
        elif spec_color := self.spec_attr("color"):
            format_rules.append(spec_color)
        elif self.node_type == NodeType.DIR:
            format_rules.append("cyan")

        # font weight
        if spec_importance := self.spec_attr("importance"):
            if spec_importance == 2:
                format_rules.append("underline")
            elif spec_importance == 1:
                format_rules.append("bold")
            elif spec_importance == -1:
                format_rules.append("dim")
        elif self.is_git_managed:
            if self.git_status == "!!":  # Git-ignored file
                format_rules.append("dim")

        # italics
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
            if self.is_loop:
                return f"[dim]@ ↺[/] [red]{self.dest_node}[/red]"
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

        name = self.name
        if self.formatted_suffix:
            name = f"{name}{self.formatted_suffix}"

        if name.startswith(".") and not args.no_align:
            name = name.replace(".", "[dim].[/dim]", 1)

        # Apply format pair.
        left, right = self.format_pair
        return f"{left}{name}{right}"

    @cached_property
    def formatted_icon(self) -> str:
        """the emoji or Nerd Font icon to show beside the node"""

        if args.icon == IconType.NONE:
            return ""

        if args.icon == IconType.EMOJI:
            icon_index = emoji_icons
        else:  # args.icon == IconType.NERD
            icon_index = nerd_icons

        if spec_icon := self.spec_attr("icon"):
            icon = icon_index.get(spec_icon)
        elif self.node_type == NodeType.DIR:
            icon = icon_index.get("folder")
        else:
            icon = None

        if icon:
            left, right = self.format_pair
            return f"{left}{icon}{right}"
        return ""

    @cached_property
    def formatted_git_status(self) -> str:
        """formatted two-letter Git status as returned by ``git-status``"""

        if self.git_status == "  ":
            return self.git_status

        format_map: dict[str, str] = {
            "M": "yellow",  # modified
            "A": "green",  # added
            "D": "red",  # deleted
            "!": "dim",  # ignored
            "-": "dim",  # padding
        }
        fmt_status = ""
        for letter in self.git_status:
            if letter == " ":
                letter = "-"
            if letter in format_map:
                fmt_status = f"{fmt_status}[{format_map[letter]}]{letter}[/]"
            else:
                fmt_status = f"{fmt_status}{letter}"
        return fmt_status

    @cached_property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        # If explicitly requested for all files, show all.
        if args.all:
            return True

        # Nodes without spec and with a leading dot are hidden.
        if not self.specs and self.name.startswith("."):
            return False

        # Nodes with importance -2 are hidden.
        if self.spec_attr("importance") == -2:
            return False

        return True

    @cached_property
    def type_char(self) -> str:
        """the single character representing the file type"""

        mapping = {
            NodeType.SYMLINK: "l",
            NodeType.DIR: "d",
            NodeType.FILE: "-",
            NodeType.FIFO: "p",
            NodeType.SOCKET: "s",
            NodeType.CHAR_DEVICE: "c",
            NodeType.BLOCK_DEVICE: "b",
        }
        return mapping[self.node_type]

    @cached_property
    def table_row(self) -> Union[dict[str, str], None]:
        """the mapping of column names and value when tabulating the node"""

        if not self.is_visible:
            return None

        cells = dict()

        name = self.formatted_name
        if not self.name.startswith(".") and not args.no_align:
            # Left pad name with a space to account for leading dots.
            name = f" {name}"
        cells["name"] = name

        cells["icon"] = self.formatted_icon

        if args.details:
            cells["type"] = self.type_char
            cells["perms"] = get_permission_text(self.stat.st_mode)
            cells["user"] = get_user(self.stat.st_uid)
            cells["group"] = get_group(self.stat.st_gid)
            if self.node_type != NodeType.DIR:
                cells["size"] = get_size(self.stat.st_size)

        if self.is_git_managed:
            cells["git"] = self.formatted_git_status

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
