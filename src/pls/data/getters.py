from __future__ import annotations

from pathlib import Path
from typing import Any, Optional

import yaml

from pls.args import args
from pls.exceptions import ConfigException
from pls.models.node_spec import NodeSpec


def load_yaml_file(file_path: Path) -> Any:
    """
    Load the YAML file referenced by the given name.

    :param file_path: the path to the YAML file to read and parse
    :return: the parsed contents of the YAML file
    """

    with file_path.open("r", encoding="utf8") as data_file:
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


def massage_specs(entry: dict) -> list[dict]:
    """
    For convenience, specs can be written as group combining several names,
    patterns or extensions. This function splits such groups into its
    constituent specs.
    Grouping does not allow mixing names, patterns and extensions.

    :param entry: a single entry from the node specs
    :return: a list of specs split from entry
    """

    id_fields = ["name", "pattern", "extension"]
    singular_plural_map = {field: f"{field}s" for field in id_fields}
    all_id_fields = [*singular_plural_map.keys(), *singular_plural_map.values()]

    # Exactly one identification method should be present.
    if [field in entry for field in all_id_fields].count(True) != 1:
        methods = ", ".join([f"`{method}`" for method in all_id_fields])
        raise ConfigException(f"Exactly one of {methods} is required.")

    # Split plurals if present.
    for singular, plural in singular_plural_map.items():
        if plural in entry:
            if not isinstance(entry[plural], list):
                raise ConfigException(
                    f"`{plural}` must be a list. Use `{singular}` instead."
                )
            common = {k: v for k, v in entry.items() if k != plural}
            return [{singular: value, **common} for value in entry[plural]]

    # Ensure no singular is list.
    for singular, plural in singular_plural_map.items():
        if singular in entry and isinstance(entry[singular], list):
            raise ConfigException(
                f"`{singular}` cannot be a list. Use `{plural}` instead."
            )

    return [entry]


def parse_node_specs(specs: list[dict]) -> list[NodeSpec]:
    """
    Parse information about the list of all node specs for all languages. This
    maps the POPO representation of the specs into the ``NodeSpec`` instances.

    :param specs: the list of POPO specs as read from the spec definition files
    :return: the list of all node specs for all languages
    """

    return [NodeSpec(**spec) for entry in specs for spec in massage_specs(entry)]


def locate_config() -> Optional[Path]:
    """
    Find a config file with the name ``.pls.yml`` in the directory or its
    ancestors, upto a max depth based on CLI arguments.

    :return: the path to the file if found, ``None`` otherwise
    """

    config_name = ".pls.yml"
    curr_dir: Path = args.directory
    for i in range(args.depth):
        config_path = curr_dir.joinpath(config_name)
        if config_path.exists() and config_path.is_file():
            return config_path
        curr_dir = curr_dir.parent
    return None


conf_data = load_yaml_file(conf_path) if (conf_path := locate_config()) else {}

node_specs: list[NodeSpec] = parse_node_specs(
    load_yaml_file(internal_yml_path("node_specs.yml"))
)
"""a list of all node specs for all languages, read from ``node_specs.yml``"""
if node_specs_ext := conf_data.get("node_specs"):
    node_specs = parse_node_specs(node_specs_ext) + node_specs

nerd_icons: dict[str, str] = load_yaml_file(internal_yml_path("nerd_icons.yml"))
"""a mapping of icon names to Unicode code-points, read from ``nerd_icons.yml``"""
if nerd_icons_ext := conf_data.get("nerd_icons"):
    nerd_icons.update(nerd_icons_ext)

emoji_icons: dict[str, str] = load_yaml_file(internal_yml_path("emoji_icons.yml"))
"""a mapping of icon names to emoji, read from ``emoji_icons.yml``"""
if emoji_icons_ext := conf_data.get("emoji_icons"):
    emoji_icons.update(emoji_icons_ext)
