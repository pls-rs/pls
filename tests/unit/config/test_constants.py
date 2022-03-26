from __future__ import annotations

import re
from pathlib import Path
from typing import Any, Callable
from unittest.mock import patch

import pytest

from pls.config.constants import get_constants
from pls.config.files import find_configs
from pls.data.utils import lookup
from pls.exceptions import ConstException
from pls.globals import state


@pytest.mark.parametrize(
    "path, expectation",
    [
        (["en"], {"a": ["apple", "animal"], "b": "ball"}),
        (["en", "a"], ["apple", "animal"]),
        (["en", "a", 0], "apple"),
        (["en", "b"], "ball"),
    ],
)
def test_lookup_dict_follows_path(path: list[str], expectation: Any):
    lookup_dict = {"en": {"a": ["apple", "animal"], "b": "ball"}}
    assert lookup(lookup_dict, path) == expectation


@pytest.mark.parametrize(
    "path",
    [
        ["es"],
        ["en", "c"],
        ["en", "a", 2],
        ["en", "a", "0"],
        ["en", ["a"]],
    ],
)
def test_lookup_dict_returns_default_or_raises_if_not_found(path: list[str]):
    lookup_dict = {"en": {"a": ["apple", "animal"], "b": "ball"}}
    assert lookup(lookup_dict, path, "default") == "default"

    path_str = ".".join([str(fragment) for fragment in path])
    with pytest.raises(ConstException, match=re.escape(path_str)):
        lookup(lookup_dict, path)


def test_union_of_constants(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        constants = get_constants(configs)

        type_chars = lookup(constants, ["type_chars"])
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

        assert lookup(constants, ["type_chars", "symlink"]) == ""
