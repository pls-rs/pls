from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls import globals
from pls.config.files import find_configs


def test_finds_all_configs_in_ancestors(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(globals.state, directory=three, git_root=None):
        configs = find_configs()
    assert configs == [path.joinpath(".pls.yml") for path in [three, two, one]]


def test_finds_config_upto_ancestor_depth(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(globals.state, directory=three, git_root=None, depth=1):
        configs = find_configs()
    assert configs == [path.joinpath(".pls.yml") for path in [three, two]]


def test_finds_config_in_git_root(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch.multiple(globals.state, directory=three, git_root=one, depth=0):
        configs = find_configs()
    assert configs == [path.joinpath(".pls.yml") for path in [three, one]]


def test_finds_config_in_home_dir(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    one, two, three = work_dirs
    for work_dir in work_dirs:
        get_conf(work_dir)

    with patch("pathlib.Path.home", return_value=one), patch.multiple(
        globals.state, directory=three, git_root=None, depth=0
    ):
        configs = find_configs()
    assert configs == [path.joinpath(".pls.yml") for path in [three, one]]
