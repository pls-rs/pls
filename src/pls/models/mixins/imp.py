from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.models.base_node import BaseNode


if TYPE_CHECKING:
    from typing import Optional


class ImpMixin(BaseNode):
    """
    Handles functionality related to importance level of the nodes.
    """

    @cached_property
    def importance(self) -> int:
        """the numerical importance level of a node"""

        imp = -2 if self.name.startswith(".") else 0
        if spec_imp := self.spec_attr("importance"):
            # If importance is set, use the given value.
            imp = spec_imp
        elif self.specs:
            # If a node has specs but not importance, importance is reset.
            imp = 0
        return imp

    @cached_property
    def importance_format(self) -> Optional[str]:
        """the formatting associated with a node's importance level"""

        importance_format_map = {-1: "dim", 1: "bold", 2: "underline"}
        return importance_format_map.get(self.importance)
