from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

from pls import globals
from pls.config.files import find_configs
from pls.config.icons import get_icons


def test_icons_union(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    get_conf(two)
    get_conf(three)

    with patch.multiple(globals.state, directory=three, git_root=None):
        configs = find_configs()
        nerd_icons, emoji_icons = get_icons(configs)

    assert set(nerd_icons.keys()) == {"mouse", "cat", "dog"}
    assert set(emoji_icons.keys()) == {"mouse", "cat", "dog"}


def test_icons_cascade(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    get_conf(two)
    get_conf(three)

    with patch.multiple(globals.state, directory=three, git_root=None):
        configs = find_configs()
        nerd_icons, emoji_icons = get_icons(configs)

    assert nerd_icons["cat"] == "ﯙ"
    assert emoji_icons["cat"] == "🐈"
