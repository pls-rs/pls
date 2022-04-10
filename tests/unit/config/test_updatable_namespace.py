import argparse
from unittest.mock import MagicMock, patch

import pytest

from pls.config.prefs import UpdatableNamespace


@pytest.mark.parametrize(
    "other, expectation",
    [
        ({"c": "c_val"}, {"a": None, "b": "b_val", "c": "c_val"}),
        ({"c": None}, {"a": None, "b": "b_val", "c": None}),
    ],
)
def test_update_adds_missing_keys(
    other: dict, expectation: dict, updatable_namespace: UpdatableNamespace
):
    other_ns = argparse.Namespace(**other)
    expectation_ns = argparse.Namespace(**expectation)

    updatable_namespace.update(other_ns)
    assert updatable_namespace == expectation_ns


@pytest.mark.parametrize(
    "other, expectation",
    [
        ({"b": "b_value"}, {"a": None, "b": "b_value"}),
        ({"b": None}, {"a": None, "b": "b_val"}),
    ],
)
def test_update_overwrites_non_null_values(
    other: dict, expectation: dict, updatable_namespace: UpdatableNamespace
):
    other_ns = argparse.Namespace(**other)
    expectation_ns = argparse.Namespace(**expectation)

    updatable_namespace.update(other_ns)
    assert updatable_namespace == expectation_ns


def test_update_handles_default(updatable_namespace: UpdatableNamespace):
    other_ns = argparse.Namespace(b="default")

    mock_internal_prefs = MagicMock(b="def_val")
    with patch("pls.config.prefs.internal_prefs", mock_internal_prefs):
        updatable_namespace.update(other_ns)

    assert updatable_namespace.b == "def_val"
