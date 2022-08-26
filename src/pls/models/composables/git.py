from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.fs.git import formatted_status
from pls.globals import state
from pls.models.format_rules import FormatRules


if TYPE_CHECKING:
    from pathlib import Path
    from typing import Optional

    from pls.models.node import Node


class GitComp:
    """
    Adds functionality related to the Git status for nodes that are Git tracked. This
    only applies to files as folders cannot be Git tracked.
    """

    def __init__(self, node: Node):
        self.node = node

        self.path_wrt_git: Optional[Path] = None
        self.git_status: Optional[str] = None

        if state.state.git_root is not None:
            try:
                self.path_wrt_git = node.path.absolute().relative_to(
                    state.state.git_root
                )
                self.git_status = state.state.git_status_map.get(self.path_wrt_git)
            except ValueError:
                # This is dest node for absolute symlink to file outside Git repo.
                pass

    @cached_property
    def cells(self) -> dict[str, str]:
        """mapping of detail keys to the corresponding formatted Git-status"""

        if state.state.git_root is None:
            return {}

        cells = {"git": formatted_status(self.git_status)}
        return cells

    @cached_property
    def format_rules(self) -> FormatRules:
        """the formatting associated with a node's Git status"""

        fmt_rules = FormatRules()

        if self.git_status == "!!":  # Git-ignored node
            fmt_rules.append("dim")

        return fmt_rules
