from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.globals import args

if TYPE_CHECKING:
    from typing import Optional, Union

    from pls.models.node import Node


class NameComp:
    """
    Adds functionality related to the name of the node.
    """

    def __init__(self, node: Node):
        self.node = node

        self.is_dotted = False
        self.pure_name = node.name
        if self.pure_name.startswith("."):
            self.is_dotted = True
            self.pure_name = self.pure_name[1:]

        self.canonical_name = self.pure_name.lower()

        self.ext: Optional[str] = None
        if "." in self.pure_name:
            self.ext = self.pure_name.split(".")[-1]

    @cached_property
    def keys(self) -> dict[str, Union[str, int, float]]:
        """mapping of detail keys to the corresponding values"""

        return {
            "name": self.canonical_name,
            "ext": self.ext if self.ext is not None else "",
        }

    @cached_property
    def display_name(self) -> str:
        """the formatted name that's seen by the user"""

        name = self.pure_name

        if self.is_dotted:
            name = f"[dim].[/]{name}" if args.args.align else f".{name}"

        name = self.node.format_rules.format_text(name)

        if args.args.align and not self.is_dotted and not self.node.is_pseudo:
            name = f" {name}"  # Pad non-dotted names with a leading space.

        return name
