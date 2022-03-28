from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING

from pls.fs.git import formatted_status
from pls.globals import state
from pls.models.base_node import BaseNode


if TYPE_CHECKING:
    from pathlib import Path
    from typing import Optional


class GitMixin(BaseNode):
    """
    Adds functionality related to Git status for objects that are Git-tracked.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.path_wrt_git: Optional[Path] = None
        self.git_status: str = "  "

        if state.state.git_root is not None:
            try:
                self.path_wrt_git = self.path.relative_to(state.state.git_root)
                self.git_status = state.state.git_status_map.get(
                    self.path_wrt_git, "  "
                )
            except ValueError:
                # This is dest node for absolute symlink to file outside Git repo.
                pass

    @cached_property
    def git_cells(self) -> dict[str, str]:
        """mapping of detail keys to the corresponding formatted Git-status"""

        if state.state.git_root is None:
            return {}

        cells = {"git": formatted_status(self.git_status)}
        return cells

    @cached_property
    def git_format_rules(self) -> tuple[list[str], list[str]]:
        """the formatting associated with a node's Git status"""

        fmt_rules: list[str] = []
        txt_fmt_rules: list[str] = []

        if self.git_status == "!!":  # Git-ignored node
            fmt_rules.append("dim")

        return fmt_rules, txt_fmt_rules
