from __future__ import annotations

import pytest

from pls.data.getters import break_plurals, massage_specs
from pls.exceptions import ConfigException


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
    ],
)
def test_massages_plurals_fields_to_singular(entry: dict, specs: list[dict]):
    assert massage_specs(entry) == specs
