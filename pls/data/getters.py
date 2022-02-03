from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

from pls.models.node_spec import NodeSpec


def load_yaml_file(file_path: Path) -> Any:
    """
    Load the YAML file referenced by the given name.

    :param file_path: the path to the YAML file to read and parse
    :return: the parsed contents of the YAML file
    """

    with open(file_path, "r") as data_file:
        data = yaml.safe_load(data_file)
    return data


def internal_yml_path(file_name: str) -> Path:
    """
    Map the given YAML data file name to the path, assuming that all the data
    files are in the same directory as the file of this function.

    :param file_name: the name of the YAML data file
    :return: the path to the YAML data file
    """

    return Path(__file__).parent.joinpath(file_name)


def split_specs(entry: dict) -> list[dict]:
    """
    For convenience, specs can be written as group combining several names,
    patterns or extensions. This function splits such groups into its
    constituent specs.
    Grouping does not allow mixing names, patterns and extensions.

    :param entry: a single entry from the node specs
    :return: a list of specs split from entry
    """

    group_fields = ["name", "pattern", "extension"]
    for field in group_fields:
        plural = f"{field}s"
        if plural in entry:
            common = {k: v for k, v in entry.items() if k != plural}
            return [{field: value, **common} for value in entry[plural]]
    return [entry]


def get_node_specs(
    file_path: Path = internal_yml_path("node_specs.yml"),
) -> list[NodeSpec]:
    """
    Parse information about the list of all node specs for all languages.
    This information is parsed from ``node_specs.yml`` unless specified.

    :param file_path: the path to the ``node_specs.yml`` file
    :return: the list of all node specs for all languages
    """

    data = load_yaml_file(file_path)
    return [NodeSpec(**spec) for entry in data for spec in split_specs(entry)]


def get_nerd_icons(
    file_path: Path = internal_yml_path("nerd_icons.yml"),
) -> dict[str, str]:
    """
    Parse information about the mapping of icon names to Unicode code-points.
    This information is parsed from ``nerd_icons.yml`` unless specified.

    :return: the mapping of icon names to Unicode code-points
    """

    return load_yaml_file(file_path)


def get_emoji_icons(
    file_path: Path = internal_yml_path("emoji_icons.yml"),
) -> dict[str, str]:
    """
    Parse information about the mapping of icon names to emoji.
    This information is parsed from ``emoji_icons.yml`` unless specified.

    :return: the mapping of icon names to emoji
    """

    return load_yaml_file(file_path)


node_specs: list[NodeSpec] = get_node_specs()
"""a list of all node specs for all languages"""

nerd_icons: dict[str, str] = get_nerd_icons()
"""a mapping of icon names to Unicode code-points"""

emoji_icons: dict[str, str] = get_emoji_icons()
"""a mapping of icon names to emoji"""

__all__ = ["node_specs", "nerd_icons", "emoji_icons"]
