from __future__ import annotations

from functools import cached_property

from pls.globals import args
from pls.models.base_node import BaseNode


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

    @property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        is_imp_visible = self.importance + args.args.all >= -1
        return is_imp_visible and super().is_visible

    @cached_property
    def importance_format_rules(self) -> tuple[list[str], list[str]]:
        """the formatting associated with a node's importance level"""

        fmt_rules: list[str] = []
        txt_fmt_rules: list[str] = []

        if self.importance <= -1:
            fmt_rules.append("dim")
        if self.importance == 1:
            txt_fmt_rules.append("bold")
        if self.importance == 2:
            txt_fmt_rules.append("underline")
        if self.importance >= 3:
            txt_fmt_rules.extend(["bold", "underline"])

        return fmt_rules, txt_fmt_rules
