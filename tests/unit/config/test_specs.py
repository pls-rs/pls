from __future__ import annotations

from pathlib import Path
from typing import Callable
from unittest.mock import patch

import pytest

from pls import globals
from pls.config.files import find_configs
from pls.config.icons import get_icons
from pls.config.specs import break_plurals, get_specs, massage_specs
from pls.exceptions import ConfigException
from pls.models.node import Node
from tests.unit.utils import strip_formatting


@pytest.mark.parametrize(
    "in_dict, singular_plural_map, out_list",
    [
        (
            {"as": [1, 2], "bs": [3, 4]},
            {"c": "cs", "a": "as"},  # uses first match
            [{"a": 1, "bs": [3, 4]}, {"a": 2, "bs": [3, 4]}],
        ),
        (
            {"as": [1, 2], "bs": [3, 4]},
            {"a": "as", "b": "bs"},  # ignores second match
            [{"a": 1, "bs": [3, 4]}, {"a": 2, "bs": [3, 4]}],
        ),
        (
            {"as": [1, 2], "bs": [3, 4]},
            {"c": "cs"},  # handles no match
            [{"as": [1, 2], "bs": [3, 4]}],
        ),
    ],
)
def test_breaks_all_plurals(
    in_dict: dict, singular_plural_map: dict[str, str], out_list: list[dict]
):
    assert break_plurals(in_dict, singular_plural_map) == out_list


@pytest.mark.parametrize(
    "entry, error_msg",
    [
        ({"a": [1]}, r"`a` cannot be a list; use `as`"),
        ({"as": 1}, r"`as` must be a list; use `a`"),
    ],
)
def test_type_mismatch_in_breaking_plurals_raises_error(entry: dict, error_msg: str):
    with pytest.raises(ConfigException, match=error_msg):
        break_plurals(entry, {"a": "as"})


@pytest.mark.parametrize(
    "entry",
    [
        # For matchers
        {"name": "name", "extension": "ext"},
        {"names": ["name"], "extensions": ["ext"]},
        {"name": "name", "extensions": ["ext"]},
        # For collapse rules
        {"name": "name", "collapse": {"name": "name", "extension": "ext"}},
        {"name": "name", "collapse": {"names": ["name"], "extensions": ["ext"]}},
        {"name": "name", "collapse": {"name": "name", "extensions": ["ext"]}},
    ],
)
def test_conflicting_fields_raises_error(entry: dict):
    with pytest.raises(ConfigException, match=r"Exactly one"):
        massage_specs(entry)


@pytest.mark.parametrize(
    "entry",
    [
        {"name": ["name"]},
        {"extension": ["ext"]},
        {"pattern": [r"pattern"]},
        {"name": "name", "collapse": {"name": ["name"]}},
        {"name": "name", "collapse": {"extension": ["ext"]}},
    ],
)
def test_massaging_plural_value_in_singular_fields_raises_error(entry: dict):
    with pytest.raises(ConfigException, match=r"`\w+` cannot be a list"):
        massage_specs(entry)


@pytest.mark.parametrize(
    "entry",
    [
        {"names": "name"},
        {"extensions": "ext"},
        {"patterns": r"pattern"},
        {"name": "name", "collapse": {"names": "name"}},
        {"name": "name", "collapse": {"extensions": "ext"}},
    ],
)
def test_massaging_singular_value_in_plural_fields_raises_error(entry: dict):
    with pytest.raises(ConfigException, match=r"`\w+` must be a list"):
        massage_specs(entry)


@pytest.mark.parametrize(
    "entry, specs",
    [
        (
            {"names": ["name_a", "name_b"]},
            [{"name": "name_a"}, {"name": "name_b"}],
        ),
        (
            {"extensions": ["ext_a", "ext_b"]},
            [{"extension": "ext_a"}, {"extension": "ext_b"}],
        ),
        (
            {"patterns": [r"pattern_a", r"pattern_b"]},
            [{"pattern": r"pattern_a"}, {"pattern": r"pattern_b"}],
        ),
        (
            {"name": "name", "collapse": {"names": ["name_a", "name_b"]}},
            [
                {"name": "name", "collapse": {"name": "name_a"}},
                {"name": "name", "collapse": {"name": "name_b"}},
            ],
        ),
        (
            {
                "names": ["name_a", "name_b"],
                "collapse": {"names": ["name_c", "name_d"]},
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
def test_massages_plurals_fields_to_singular(entry: dict, specs: list[dict]):
    assert massage_specs(entry) == specs


def test_specs_union(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    get_conf(two)
    get_conf(three)

    with patch.multiple(globals.state, directory=three, home_dir=None, git_root=None):
        configs = find_configs()
        node_specs = get_specs(configs)

    assert len(node_specs) == 2


def test_specs_cascade(
    work_dirs: tuple[Path, Path, Path], get_conf: Callable[[Path], Path]
):
    _, two, three = work_dirs
    get_conf(two)
    get_conf(three)

    with patch.multiple(globals.state, directory=three, home_dir=None, git_root=None):
        configs = find_configs()
        node_specs = get_specs(configs)
        nerd_icons, _ = get_icons(configs)
    with patch("pls.models.node.nerd_icons", nerd_icons):
        test_node = Node(name="cat.py", path=three.joinpath("cat.py"))
        test_node.match_specs(node_specs)
        icon = test_node.formatted_icon

    assert strip_formatting(icon) == "ﯙ"
    assert test_node.importance == 1
