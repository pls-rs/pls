from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING


if TYPE_CHECKING:
    from pathlib import Path
    from typing import Any

    from pls.models.node_spec import NodeSpec


class BaseNode:
    """
    Acts as the final node in the MRO for the ``__init__`` calls.
    """

    def __init__(self, name: str, path: Path):
        self.name = name
        self.path = path

        self.specs: list[NodeSpec] = []  # matched in ``match_specs``

    @cached_property
    def pure_name(self) -> str:
        """the name of the node with the leading dot stripped"""

        if self.name.startswith("."):
            return self.name.replace(".", "", 1)
        else:
            return self.name

    @cached_property
    def canonical_name(self) -> str:
        """the case-normalised pure name of the node"""

        return self.pure_name.lower()

    @cached_property
    def extension(self) -> str:
        """the extension of the node, i.e. the portion after the last dot"""

        return self.name.split(".")[-1] if "." in self.name else ""

    def match_specs(self, specs: list[NodeSpec]):
        """
        Find all spec matching this node from a list of all possible specs and
        store them in the ``specs`` attribute.

        :param specs: the list of all specs
        """

        self.specs = [spec for spec in specs if spec.match(self)]

    def spec_attr(self, attr: str) -> Any:
        """
        Get the requested attribute from the first matching spec to provide it.

        :param attr: the requested attribute
        :return: the value of the attribute if found, ``None`` otherwise
        """

        for spec in self.specs:
            if attr_val := getattr(spec, attr, None):
                return attr_val
        return None
