from __future__ import annotations

from pathlib import Path
from typing import Any, Union

import yaml

from pls.args import args
from pls.models.node_spec import NodeSpec


def load_yaml_file(file_path: Path) -> Any:
    """
    Load the YAML file referenced by the given name.

    :param file_path: the path to the YAML file to read and parse
    :return: the parsed contents of the YAML file
    """

    with file_path.open("r") as data_file:
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
            if type(entry[plural]) != list:
                raise ValueError(f"`{field}s` must be a list. Use `{field}`.")
            common = {k: v for k, v in entry.items() if k != plural}
            return [{field: value, **common} for value in entry[plural]]
    return [entry]


def parse_node_specs(specs: list[dict]) -> list[NodeSpec]:
    """
    Parse information about the list of all node specs for all languages. This
    maps the POPO representation of the specs into the ``NodeSpec`` instances.

    :param specs: the list of POPO specs as read from the spec definition files
    :return: the list of all node specs for all languages
    """

    return [NodeSpec(**spec) for entry in specs for spec in split_specs(entry)]


def locate_extension() -> Union[Path, None]:
    """
    Find a config file with the name ``.pls.yml`` in the directory or its
    ancestors, upto a max depth based on CLI arguments.

    :return: the path to the file if found, ``None`` otherwise
    """

    extension_name = ".pls.yml"
    curr_dir: Path = args.directory
    for i in range(args.depth):
        extension_path = curr_dir.joinpath(extension_name)
        if extension_path.exists() and extension_path.is_file():
            return extension_path
        curr_dir = curr_dir.parent
    return None


ext_data = load_yaml_file(ext_path) if (ext_path := locate_extension()) else {}

node_specs: list[NodeSpec] = parse_node_specs(
    load_yaml_file(internal_yml_path("node_specs.yml"))
)
"""a list of all node specs for all languages, read from ``node_specs.yml``"""
if node_specs_ext := ext_data.get("node_specs"):
    node_specs = parse_node_specs(node_specs_ext) + node_specs

nerd_icons: dict[str, str] = load_yaml_file(internal_yml_path("nerd_icons.yml"))
"""a mapping of icon names to Unicode code-points, read from ``nerd_icons.yml``"""
if nerd_icons_ext := ext_data.get("nerd_icons"):
    nerd_icons.update(nerd_icons_ext)

emoji_icons: dict[str, str] = load_yaml_file(internal_yml_path("emoji_icons.yml"))
"""a mapping of icon names to emoji, read from ``emoji_icons.yml``"""
if emoji_icons_ext := ext_data.get("emoji_icons"):
    emoji_icons.update(emoji_icons_ext)

__all__ = ["node_specs", "nerd_icons", "emoji_icons"]
