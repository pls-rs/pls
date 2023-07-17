from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING, Callable

from rich.markup import escape

from pls.config import icons
from pls.enums.icon_type import IconType
from pls.globals import args
from pls.models.composables.children import ChildrenComp
from pls.models.composables.collapse import CollapseComp
from pls.models.composables.git import GitComp
from pls.models.composables.imp import ImpComp
from pls.models.composables.name import NameComp
from pls.models.composables.spec import SpecComp
from pls.models.composables.stat import StatComp
from pls.models.composables.type import TypeComp
from pls.models.format_rules import FormatRules
from pls.models.tree import Tree


if TYPE_CHECKING:
    from pathlib import Path
    from typing import Optional, Union

    from pls.models.node_spec import NodeSpec


class Node(Tree):
    """
    A node is any file, folder or symlink on the file-system. This model stores
    attributes pertaining to a single FS node. Nodes are read from the file
    system directly.
    """

    def __init__(self, name: str, path: Path, is_pseudo: bool = False):
        super().__init__()

        self._name = name
        self.name = escape(name)
        self.path = path
        self.is_pseudo = is_pseudo  # true for symlink destinations

        self.name_comp = NameComp(self)
        self.git_comp = GitComp(self)
        self.imp_comp = ImpComp(self)
        self.stat_comp = StatComp(self)
        self.spec_comp = SpecComp(self)
        self.type_comp = TypeComp(self)
        self.collapse_comp = CollapseComp(self)
        self.children_comp = ChildrenComp(self)

    def __eq__(self, other: object) -> bool:
        """
        Compare the object ``self`` to any other object.

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
    def format_rules(self) -> FormatRules:
        """the list of formatting rules to apply to the icons and text respectively"""

        fmt_rules = FormatRules()

        for rules in [
            self.type_comp.format_rules,
            self.git_comp.format_rules,
            self.imp_comp.format_rules,
        ]:
            fmt_rules.extend(rules)

        # font color
        if spec_color := self.spec_comp.attr("color"):
            fmt_rules.append(str(spec_color))

        return fmt_rules

    @cached_property
    def formatted_name(self) -> str:
        """the name, formatted using Rich console formatting markup"""

        name = self.name_comp.display_name

        if suffix := self.type_comp.display_suffix:
            name = f"{name}{suffix}"

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

        icon = None
        if type_icon := self.type_comp.icon:
            icon = type_icon
        if spec_icon := self.spec_comp.attr("icon"):
            icon = spec_icon

        if icon is not None and (icon := icon_index.get(icon)):
            return self.format_rules.format_icon(icon)
        return ""

    def populate_tree(
        self,
        specs: list[NodeSpec],
        populate_callback: Callable = lambda _: None,
    ):
        """
        Populate nodes with their children for a treeview preview, recursively.

        :param specs: list of NodeSpec objects
        :param populate_callback: callback to populate the Tree
        """

        if not self.is_visible:
            return

        self.spec_comp.match(specs)
        self.children_comp.find_children()

        self.set_sub_pre_shapes()

        populate_callback(self)

        for child in self.children:
            child.populate_tree(specs=specs, populate_callback=populate_callback)

    # Composition aggregations
    # ========================

    @cached_property
    def is_visible(self) -> bool:
        """whether the node is supposed to be displayed"""

        return all(
            [
                self.imp_comp.is_visible,
                self.type_comp.is_visible,
                self.collapse_comp.is_visible,
            ]
        )

    @cached_property
    def table_row(self) -> Optional[dict[str, Optional[str]]]:
        """the mapping of column names and value when tabulating the node"""

        if not self.is_visible:  # No table row for invisible nodes.
            return None

        cells: dict[str, Optional[str]] = {
            "name": self.formatted_name,
            "icon": self.formatted_icon,
        }

        if not args.args.details:
            return cells  # Return early because no more data is needed.

        for cells_item in [
            self.type_comp.cells,
            self.stat_comp.cells,
            self.git_comp.cells,
        ]:
            cells.update(cells_item)

        return cells

    @cached_property
    def sort_keys(self) -> dict[str, Union[str, int, float]]:
        """the mapping of sort fields to their normalised values"""

        keys: dict[str, Union[str, int, float]] = {}

        for keys_item in [
            self.name_comp.keys,
            self.type_comp.keys,
            self.stat_comp.keys,
        ]:
            keys.update(keys_item)

        return keys
