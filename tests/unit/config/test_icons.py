from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls.config.files import find_configs
from pls.config.icons import get_icons
from pls.globals import state


def test_union_of_icons(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        nerd_icons, emoji_icons = get_icons(configs)

    assert set(nerd_icons.keys()) == {"mouse", "cat", "dog"}
    assert set(emoji_icons.keys()) == {"mouse", "cat", "dog"}


def test_inner_icons_override_outer(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        nerd_icons, emoji_icons = get_icons(configs)

    assert nerd_icons["cat"] == "ﯙ"
    assert emoji_icons["cat"] == "🐈"
