"""
Do not import variables from this module, always import the module directly and use its
variables with a dot ``.`` notation.

This module must be synchronised with ``pls.args`` due to the overlapping nature of
CLI args and config-based preferences.
"""

from __future__ import annotations

import argparse
from enum import Enum
from pathlib import Path
from typing import Type, Union

from pls.data.utils import load_yml_file
from pls.enums.icon_type import IconType
from pls.enums.unit_system import UnitSystem
from pls.exceptions import ConfigException


def _parse_enums(preferences: dict):
    """
    This function reads the preferences dictionary and for all values that are supposed
    to be enums, invokes the enum constructor on them to convert them to the enum type.

    :param preferences: the dictionary of preferences in which to parse enums
    """

    enum_map: dict[str, Type[Enum]] = {
        "icon": IconType,
        "units": UnitSystem,
    }
    for pref, val in preferences.items():
        if pref in enum_map:
            try:
                preferences[pref] = enum_map[pref](val)
            except ValueError as exc:
                raise ConfigException(
                    f"Invalid value '{val}' for preference [italic]`{pref}`[/]."
                ) from exc


def get_prefs(conf_paths: Union[Path, list[Path]]) -> argparse.Namespace:
    """
    Prefs are namespaces parsed from dictionaries that match the CLI args and provide a
    way to codify repetitive CLI arguments in the ``.pls.yml`` config files.

    :param conf_paths: the list of config files from which to import icons
    :return: the parsed preference namespaces
    """

    preferences: dict = {}
    if not isinstance(conf_paths, list):
        conf_paths = [conf_paths]

    for conf_path in reversed(conf_paths):
        conf = load_yml_file(conf_path)

        pref_val = conf.get("prefs", {})
        if not pref_val:
            continue
        if not isinstance(pref_val, dict):
            raise ConfigException("[italic]`prefs`[/] must be a dictionary.")
        preferences.update(pref_val)

    _parse_enums(preferences)
    return argparse.Namespace(**preferences)


internal_prefs: argparse.Namespace
"""the preferences read from the internal ``prefs.yml`` file"""

prefs: argparse.Namespace
"""the preferences read from the ``pls`` config files"""
