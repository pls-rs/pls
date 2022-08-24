from __future__ import annotations

import logging
from typing import Literal, Optional, Tuple, cast, get_args

import requests
from requests import RequestException

from pls import __pkg__, __version__
from pls.globals import console


logger = logging.getLogger(__name__)

VersionTriplet = Tuple[int, int, int]
UpgradeType = Literal["major", "minor", "patch"]


def parse_semver(version: str) -> VersionTriplet:
    """
    Break a version string into a triplet of its constituent parts.

    :param version: the version string
    :return: the triplet of version parts
    """

    version_parts = version.split(".")
    return cast(VersionTriplet, tuple(int(part) for part in version_parts))


def compare_versions(old: VersionTriplet, new: VersionTriplet) -> Optional[UpgradeType]:
    """
    Compare two version triplets and determine the kind of upgrade is needed to go from
    one to the other.

    :param old: the older version
    :param new: the current version
    :return: the nature of the upgrade from the older version to the newer one
    """

    upgrade_types = get_args(UpgradeType)
    for new_part, old_part, upgrade_type in zip(new, old, upgrade_types):
        if new_part > old_part:
            return upgrade_type
    return None


def get_latest_version() -> str:
    """
    Get the latest version of ``pls`` from PyPI.

    :return: the version triplet of the latest version on PyPI
    """

    res = requests.get(f"https://pypi.org/pypi/{__pkg__}/json", timeout=0.5)
    package_info = res.json()
    return package_info["info"]["version"]


def print_version():
    """
    Print the current version to the console.
    """

    console.console.print(f"[red bold]pls[/] [blue]{__version__}[/]")


def check_update():
    """
    Compare the current version to the latest and show an upgrade message if a newer
    version has been published.
    """

    try:
        latest_version = get_latest_version()
    except RequestException:
        return

    latest_ver = parse_semver(latest_version)
    curr_ver = parse_semver(__version__)

    if diff := compare_versions(curr_ver, latest_ver):
        upgrade_color_map: dict[UpgradeType, str] = {
            "major": "red",
            "minor": "yellow",
            "patch": "green",
        }
        message_color = upgrade_color_map[diff]

        console.console.print(
            f"A new [bold {message_color}]{diff}[/] version is available: "
            f"[blue]{latest_version}[/]. "
            "Please upgrade."
        )
