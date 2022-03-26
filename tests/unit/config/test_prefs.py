from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls.config.files import find_configs
from pls.config.prefs import get_prefs
from pls.globals import state


def test_union_of_prefs(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        prefs = get_prefs(configs)

    assert set(vars(prefs).keys()) == {"sort", "dirs_first", "time_fmt"}


def test_inner_prefs_override_outer(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        prefs = get_prefs(configs)

    assert prefs.dirs_first
