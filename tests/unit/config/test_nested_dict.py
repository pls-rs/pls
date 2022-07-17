from __future__ import annotations

import re
from typing import Any

import pytest

from pls.config.constants import NestedDict
from pls.exceptions import ConstException


@pytest.mark.parametrize(
    "other, overwrite, expectation",
    [
        (
            {"dict": {"b": 2}, "list": ["b"], "k": "v"},
            False,
            {
                "dict": {"a": 1, "b": 2},
                "list": ["a", "b"],
                "scalar": "val",
                "k": "v",
                "null": None,
            },
        ),
        (
            {"dict": {"b": 2}, "list": ["b"], "scalar": "value"},
            True,
            {
                "dict": {"a": 1, "b": 2},
                "list": ["a", "b"],
                "scalar": "value",
                "null": None,
            },
        ),
    ],
)
def test_nested_dict_supports_merge(
    other: dict, overwrite: bool, expectation: dict, nested_dict: NestedDict
):
    nested_dict.deep_merge(other, overwrite)
    assert nested_dict == expectation


@pytest.mark.parametrize(
    "other, path",
    [
        ({"dict": {"a": 2}}, "dict.a"),
        ({"scalar": "value"}, "scalar"),
    ],
)
def test_nested_dict_merge_raises_if_conflict(
    other: dict, path: str, nested_dict: NestedDict
):
    with pytest.raises(ConstException, match=re.escape(path)):
        nested_dict.deep_merge(other)


@pytest.mark.parametrize(
    "path, expectation",
    [
        (["dict"], {"a": 1}),
        (["dict", "a"], 1),
        (["list"], ["a"]),
        (["list", 0], "a"),
        (["scalar"], "val"),
        (["null"], None),
    ],
)
def test_nested_dict_supports_lookup(
    path: list[str], expectation: Any, nested_dict: NestedDict
):
    assert nested_dict.lookup(*path) == expectation


@pytest.mark.parametrize(
    "path",
    [
        ["dne"],
        ["dict", "b"],
        ["list", 1],
        ["list", "0"],
    ],
)
def test_nested_dict_lookup_returns_default_or_raises_if_not_found(
    path: list[str], nested_dict: NestedDict
):
    assert nested_dict.lookup(*path, default="default") == "default"

    path_str = ".".join([str(fragment) for fragment in path])
    with pytest.raises(ConstException, match=re.escape(path_str)):
        nested_dict.lookup(*path)


def test_nested_dict_lookup_returns_null_if_null(nested_dict: NestedDict):
    assert nested_dict.lookup("null", default="default") is None
