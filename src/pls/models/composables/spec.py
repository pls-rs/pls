from __future__ import annotations

from typing import TYPE_CHECKING

from pls.globals import args
from pls.models.node_spec import NodeSpec


if TYPE_CHECKING:
    from typing import Any

    from pls.models.node import Node


class SpecComp:
    """
    Adds functionality related to matching and extracting information from the specs.
    """

    def __init__(self, node: Node):
        self.node = node

        self.specs: list[NodeSpec] = []  # matched in ``match``

    def match(self, specs: list[NodeSpec]):
        """
        Find all spec matching this node from a list of all possible specs and
        store them in the ``specs`` attribute.

        :param specs: the list of all specs
        """

        self.specs = [spec for spec in specs if spec.match(self.node)]

    def attr(self, attr: str, coalesce: bool = False) -> Any:
        """
        Get the requested attribute from the first matching spec to provide it.

        :param attr: the requested attribute
        :param coalesce: whether to group attrs from all specs and return a list
        :return: the value of the attribute if found, ``None`` otherwise
        """

        values = []
        for spec in self.specs:
            if attr_val := getattr(spec, attr, None):
                if coalesce:
                    values.append(attr_val)
                else:
                    return attr_val
        return values if coalesce else None
