from __future__ import annotations

from typing import Union

from pls.args import args
from pls.data.getters import emoji_icons, nerd_icons
from pls.enums.icon_type import IconType
from pls.enums.node_type import NodeType
from pls.models.node_spec import NodeSpec


class Node:
    """
    A node is any file, folder or symlink on the file-system. This model stores
    attributes pertaining to a single FS node.

    Nodes are read from the file system directly using ``os.walk``.
    """

    def __init__(self, name: str, node_type: NodeType):
        self.name = name
        self.node_type = node_type

        self.specs: list[NodeSpec] = []  # matched later (see ``map_specs``)

    def __repr__(self):
        """
        Get the string representation of the ``Node`` instance. This is also
        used by ``__str__`` automatically.

        :return: the string representation
        """

        name = self.name
        if self.node_type == NodeType.FOLDER:
            name = f"{name}/"

        return name

    @property
    def ext(self) -> Union[str, None]:
        """the extension of the node, i.e. the portion after the last dot"""

        return self.name.split(".")[-1] if "." in self.name else None

    @property
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
        elif self.node_type == NodeType.FOLDER:
            icon = icon_index.get("folder")
        else:
            icon = None
        return icon or ""

    @property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        # Nodes without spec and with a leading dot are hidden
        if not self.specs and self.name.startswith("."):
            return False

        # Nodes with importance -2 are hidden
        if self.spec_attr("importance") == -2:
            return False

        return True

    @property
    def formatted_name(self) -> str:
        """the name, formatted using Rich console formatting markup"""

        name = str(self)
        if not args.no_align:
            if name.startswith("."):
                name = name.replace(".", "[dim].[/dim]", 1)
            else:
                name = f" {name}"
        return name

    @property
    def format_pair(self) -> tuple[str, str]:
        """the opening and closing tags of Rich console formatting markup"""

        format_rules = []

        # Font color
        if spec_color := self.spec_attr("color"):
            format_rules.append(spec_color)
        elif self.node_type == NodeType.FOLDER:
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
            right = f"[/{' '.join(format_rules[::-1])}]"
        else:
            left = right = ""
        return left, right

    @property
    def table_row(self) -> list[str]:
        """the list of cells when presenting the node as a row in a table"""

        cells = [self.formatted_name]
        if args.icon != IconType.NONE:
            cells.insert(0, self.icon)

        left, right = self.format_pair
        return [f"{left}{cell}{right}" for cell in cells]

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
