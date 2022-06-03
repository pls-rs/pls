from __future__ import annotations

from typing import Optional

import pytest

from pls.output.update import (
    UpgradeType,
    VersionTriplet,
    compare_versions,
    parse_semver,
)


@pytest.mark.parametrize(
    "version, triplet",
    [
        ("1.0.0", (1, 0, 0)),
        ("1.10.0", (1, 10, 0)),  # handles two digit parts
        ("1.00.000", (1, 0, 0)),  # handles multiple zeros
    ],
)
def test_parse_semver(version: str, triplet: VersionTriplet):
    assert parse_semver(version) == triplet


@pytest.mark.parametrize(
    "old, new, diff",
    [
        ((1, 1, 1), (1, 1, 1), None),
        ((1, 1, 1), (1, 1, 2), "patch"),
        ((1, 1, 1), (1, 2, 2), "minor"),
        ((1, 1, 1), (2, 2, 2), "major"),
    ],
)
def test_compare_versions(
    old: VersionTriplet, new: VersionTriplet, diff: Optional[UpgradeType]
):
    assert compare_versions(old, new) == diff
