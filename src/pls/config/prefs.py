"""
Do not import variables from this module, always import the module directly and use its
variables with a dot ``.`` notation.

This module must be synchronised with ``pls.args`` due to the overlapping nature of
CLI args and config-based preferences.
"""

from __future__ import annotations

import argparse
import copy
import logging
import re
from enum import Enum
from pathlib import Path
from typing import Type, Union

from pls.data.utils import load_yml_file
from pls.enums.icon_type import IconType
from pls.enums.unit_system import UnitSystem
from pls.exceptions import ConfigException


logger = logging.getLogger(__name__)


class UpdatableNamespace(argparse.Namespace):
    """
    Extends ``argparse.Namespace`` to add support for overwriting attributes from
    another ``argparse.Namespace`` instance.
    """

    def update(self, more: argparse.Namespace):
        """
        Overwrite own attributes with attributes from another namespace.

        :param more: the namespace from which to read the attributes
        """

        logger.info("Updating namespace")

        logger.debug(f"Current: {self}")
        logger.debug(f"Incoming: {more}")

        for key, val in vars(more).items():
            if key not in self or val is not None:
                if val == "default":
                    val = getattr(internal_prefs, key, None)
                setattr(self, key, val)

        logger.debug(f"Result: {self}")


def _parse_enums(preferences: dict):
    """
    This function reads the preferences dictionary and for all values that are supposed
    to be enums, invokes the enum constructor on them to convert them to the enum type.

    :param preferences: the dictionary of preferences in which to parse enums
    """

    enum_fields_map: dict[str, Type[Enum]] = {
        "icon": IconType,
        "units": UnitSystem,
    }
    for field, enum_class in enum_fields_map.items():
        if field not in preferences:
            continue
        val = preferences[field]
        try:
            preferences[field] = enum_class(val)
        except ValueError as exc:
            raise ConfigException(
                f"Invalid value '{val}' for preference [italic]`{field}`[/]."
            ) from exc


def _parse_lists(preferences: dict):
    """
    This function reads the preferences dictionary and for all values that are supposed
    to be list fields, processes them using logic similar to the ``StoreOrCountAction``
    defined for ``argparse``.

    :param preferences: the dictionary of preferences in which to parse lists
    """

    list_field_list = ["details"]
    for field in list_field_list:
        if field not in preferences:
            continue
        values = preferences[field]
        parsed_values: list[str] = []
        for value in values:
            if value == "none":
                parsed_values = []
            else:
                parsed_values.append(value)
        preferences[field] = parsed_values


def _parse_regexes(preferences: dict):
    """
    This function reads the preferences dictionary and for all values that are supposed
    to be compiled regular expressions, processes them using ``re.compile``.

    :param preferences: the dictionary of preferences in which to parse regexes
    """

    regex_field_list = ["exclude", "only"]
    for field in regex_field_list:
        if field not in preferences:
            continue
        value = preferences[field]
        preferences[field] = re.compile(value)


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
        # Use a copy to prevent ``load_yml_file`` cache from being polluted.
        conf = copy.deepcopy(load_yml_file(conf_path))

        pref_val = conf.get("prefs", {})
        if not pref_val:
            continue
        if not isinstance(pref_val, dict):
            raise ConfigException("[italic]`prefs`[/] must be a dictionary.")
        preferences.update(pref_val)

    _parse_enums(preferences)
    _parse_lists(preferences)
    _parse_regexes(preferences)

    return argparse.Namespace(**preferences)


internal_prefs: argparse.Namespace
"""the preferences read from the internal ``prefs.yml`` file"""

config_prefs: argparse.Namespace
"""the preferences read from the ``pls`` config files"""
