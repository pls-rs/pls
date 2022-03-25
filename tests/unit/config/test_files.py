from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls.config.files import find_configs, get_ancestor_confs
from pls.globals import state


def test_finds_ancestor_configs_till_git_root(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(state.state, git_root=one):
        configs = get_ancestor_confs(three)
    assert configs == [path.joinpath(".pls.yml") for path in [two, one]]


def test_finds_ancestor_configs_till_max_height(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(state.state, git_root=None), patch(
        "pls.config.files.max_height", 1
    ):
        configs = get_ancestor_confs(three)
    assert configs == [path.joinpath(".pls.yml") for path in [two]]


def test_adds_home_conf_if_missing(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=one, git_root=None), patch(
        "pls.config.files.max_height", 0
    ):
        configs = find_configs(three)
    assert configs == [path.joinpath(".pls.yml") for path in [three, one]]


def test_leaves_home_conf_if_present(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=one, git_root=None), patch(
        "pls.config.files.max_height", 2
    ):
        configs = find_configs(three)
    assert configs == [path.joinpath(".pls.yml") for path in [three, two, one]]
