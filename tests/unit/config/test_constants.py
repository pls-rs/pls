from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls.config.constants import get_constants
from pls.config.files import find_configs
from pls.globals import state


def test_union_of_constants(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        constants = get_constants(configs)

        type_chars = constants.lookup("type_chars")
        assert set(type_chars.keys()) == {"symlink", "fifo", "dir"}


def test_inner_constants_override_outer(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        constants = get_constants(configs)

        assert constants.lookup("type_chars", "symlink") == ""
