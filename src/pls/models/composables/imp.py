from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.globals import args
from pls.models.format_rules import FormatRules

if TYPE_CHECKING:
    from pls.models.node import Node


class ImpComp:
    """
    Adds functionality related to the importance level of the nodes.
    """

    def __init__(self, node: Node):
        self.node = node

    @staticmethod
    def default_importance(node: Node):
        """
        Get the default importance level for a node based on two factors. Refer to the
        `docs <https://dhruvkb.github.io/pls/features/importance.html#reference>`_ for
        more info.

        :param node: the node for which to get the default importance
        :return: the default importance level
        """

        has_specs = len(node.spec_comp.specs) != 0
        has_leading_dot = node.name.startswith(".")
        importance_matrix = {
            # (has_leading_dot, has_specs)
            (False, True): 0,
            (False, False): 0,
            (True, True): -1,
            (True, False): -2,
        }
        return importance_matrix[(has_leading_dot, has_specs)]

    @cached_property
    def importance(self):
        return (
            spec_imp
            if (spec_imp := self.node.spec_comp.attr("importance"))
            else self.default_importance(self.node)
        )

    @cached_property
    def format_rules(self) -> FormatRules:
        """the formatting associated with a node's importance level"""

        fmt_rules = FormatRules()

        if self.importance <= -1:
            fmt_rules.append("dim")
        if self.importance == 1:
            fmt_rules.append("bold")
        if self.importance == 2:
            fmt_rules.append("underline")
        if self.importance >= 3:
            fmt_rules.extend(["bold", "underline"])

        return fmt_rules

    @cached_property
    def is_visible(self) -> bool:
        """whether the node deserves to be rendered to the screen"""

        is_imp_visible = self.importance + args.args.all >= -1
        return is_imp_visible
