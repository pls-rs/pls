from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.globals import state
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

        imp = -1 if self.specs else -2 if self.name.startswith(".") else 0
        if spec_imp := self.spec_attr("importance"):
            # If importance is set, use the given value.
            imp = spec_imp
        elif self.specs:
            # If a node has specs but not importance, importance is reset.
            imp = 0
        return imp

    @cached_property
    def is_visible_imp(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        return self.importance + state.state.all >= -1

    @cached_property
    def importance_format(self) -> Optional[str]:
        """the formatting associated with a node's importance level"""

        if self.importance <= -1:
            return "dim"
        if self.importance == 1:
            return "bold"
        if self.importance == 2:
            return "underline"
        if self.importance >= 3:
            return "bold underline"
        return None
