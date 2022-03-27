from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

import pytest

from pls.config.files import find_configs
from pls.config.icons import get_icons
from pls.config.specs import break_plurals, get_specs, massage_specs
from pls.exceptions import ConfigException
from pls.globals import state
from pls.models.node import Node
from pls.utils.strip_fmt import strip_formatting


@pytest.mark.parametrize(
    "in_dict, split_keys, out_list",
    [
        (
            {"a": [1, 2], "b": [3, 4]},
            ["c", "a"],  # uses first match
            [{"a": 1, "b": [3, 4]}, {"a": 2, "b": [3, 4]}],
        ),
        (
            {"a": [1, 2], "b": [3, 4]},
            ["a", "b"],  # ignores second match
            [{"a": 1, "b": [3, 4]}, {"a": 2, "b": [3, 4]}],
        ),
        (
            {"a": [1, 2], "b": [3, 4]},
            ["c"],  # handles no match
            [{"a": [1, 2], "b": [3, 4]}],
        ),
    ],
)
def test_breaks_all_plurals(in_dict: dict, split_keys: list[str], out_list: list[dict]):
    assert break_plurals(in_dict, split_keys) == out_list


@pytest.mark.parametrize(
    "entry",
    [
        # For matchers
        {"name": "name", "extension": "ext"},
        # For collapse rules
        {"name": "name", "collapse": {"name": "name", "extension": "ext"}},
    ],
)
def test_conflicting_fields_raises_error(entry: dict):
    with pytest.raises(ConfigException, match=r"Exactly one"):
        massage_specs(entry)


@pytest.mark.parametrize(
    "entry",
    [
        {"name": "name"},
        {"extension": "ext"},
        {"pattern": r"pattern"},
        {"name": "name", "collapse": {"name": "name"}},
        {"name": "name", "collapse": {"extension": "ext"}},
    ],
)
def test_massaging_singular_value_has_no_effect(entry: dict):
    assert massage_specs(entry) == [entry]


@pytest.mark.parametrize(
    "entry, specs",
    [
        (
            {"name": ["name_a", "name_b"]},
            [{"name": "name_a"}, {"name": "name_b"}],
        ),
        (
            {"extension": ["ext_a", "ext_b"]},
            [{"extension": "ext_a"}, {"extension": "ext_b"}],
        ),
        (
            {"pattern": [r"pattern_a", r"pattern_b"]},
            [{"pattern": r"pattern_a"}, {"pattern": r"pattern_b"}],
        ),
        (
            {"name": "name", "collapse": {"name": ["name_a", "name_b"]}},
            [
                {"name": "name", "collapse": {"name": "name_a"}},
                {"name": "name", "collapse": {"name": "name_b"}},
            ],
        ),
        (
            {
                "name": ["name_a", "name_b"],
                "collapse": {"name": ["name_c", "name_d"]},
            },
            [
                {"name": "name_a", "collapse": {"name": "name_c"}},
                {"name": "name_a", "collapse": {"name": "name_d"}},
                {"name": "name_b", "collapse": {"name": "name_c"}},
                {"name": "name_b", "collapse": {"name": "name_d"}},
            ],
        ),
    ],
)
def test_massaging_plurals_values_splits_entry(entry: dict, specs: list[dict]):
    assert massage_specs(entry) == specs


def test_specs_union(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        node_specs = get_specs(configs)

    assert len(node_specs) == 2


def test_inner_specs_override_outer(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    for work_dir in [two, three]:
        get_conf(work_dir)

    with patch.multiple(state.state, home_dir=None, git_root=two):
        configs = find_configs(three)
        node_specs = get_specs(configs)
        nerd_icons, _ = get_icons(configs)
    with patch("pls.config.icons.nerd_icons", nerd_icons):
        test_node = Node(name="cat.py", path=three.joinpath("cat.py"))
        test_node.match_specs(node_specs)
        icon = test_node.formatted_icon

    assert strip_formatting(icon) == "ﯙ"
    assert test_node.importance == 1
