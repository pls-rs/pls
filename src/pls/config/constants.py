"""
Do not import variables from this module, always import the module directly and use its
variables with a dot ``.`` notation.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from pls.data.utils import deep_merge, load_yml_file
from pls.exceptions import ConfigException


def get_constants(conf_paths: list[Path]) -> dict[str, Any]:
    """
    Constants are mappings of names and values that ``pls`` treats as hardcoded values.
    Changing constants acts as a subtle way to tweak the output.

    :param conf_paths: the list of config files from which to import constants
    :return: the mapping of icon name to icon glyph
    """

    consts: dict[str, Any] = {}

    for conf_path in reversed(conf_paths):
        conf = load_yml_file(conf_path)

        consts_val = conf.get("constants", {})
        if not consts_val:
            continue
        if not isinstance(consts_val, dict):
            raise ConfigException("[italic]`constants`[/] must be a dictionary.")
        deep_merge(consts, consts_val, overwrite=True)

    return consts


constants: dict[str, Any]
"""the mapping of icon names to Unicode code-points"""
