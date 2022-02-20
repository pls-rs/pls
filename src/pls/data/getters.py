from __future__ import annotations

from pathlib import Path
from typing import Optional

from pls.args import args
from pls.data.utils import internal_yml_path, load_yaml_file
from pls.exceptions import ConfigException
from pls.models.node_spec import NodeSpec


def break_plurals(entry: dict, singular_plural_map: dict[str, str]) -> list[dict]:
    """
    Split a dict with a key containing a list into separate dictionaries, with
    the singular form of the key containing one item of the list.

    The function checks against:

    - singular keys containing a list
    - plural keys not containing a list

    :param entry: the dict to split from plural to singular
    :param singular_plural_map: the map of singular and plural keys
    :return: a list of dictionaries from splitting the plurals
    :raise: ``ConfigException``, if the fields have the wrong type of value
    """

    for singular, plural in singular_plural_map.items():
        if singular_value := entry.get(singular):
            # Singular keys cannot not contain a list.
            if isinstance(singular_value, list):
                raise ConfigException(
                    f"`{singular}` cannot be a list; use `{plural}`: {entry}"
                )

        if plural_value := entry.get(plural):
            # Plural keys should contain a list.
            if not isinstance(plural_value, list):
                raise ConfigException(
                    f"`{plural}` must be a list; use `{singular}`: {entry}"
                )

            common = {k: v for k, v in entry.items() if k != plural}
            return [{singular: item, **common} for item in plural_value]

    # If no match, wrap the entry and return as-is.
    return [entry]


def check_conflicts(entry: dict, conflict_keys: list[str]):
    """
    Check for the presence of conflicting keys in the given dict. If more or
    less than one key is present in the dict, raise a ``ConfigException``.

    :param entry: the entry to check for conflict
    :param conflict_keys: the list of mutually exclusive keys
    :raise: ``ConfigException``, if conflicting keys are present
    """

    if [field in entry for field in conflict_keys].count(True) != 1:
        fields = ", ".join([f"`{field}`" for field in conflict_keys])
        raise ConfigException(f"Exactly one of {fields} is allowed: {entry}.")


def massage_specs(entry: dict) -> list[dict]:
    """
    For convenience, specs can be written as groups combining several names,
    patterns or extensions. This function splits such groups into its
    constituent specs.

    Grouping does not allow mixing multiple forms of identification. This
    function checks against mixing multiple modes of identification.

    :param entry: a single entry from the node specs
    :return: a list of specs split from entry
    """

    # Split collapse names/extensions into collapses name/extension.
    if collapse := entry.get("collapse"):
        collapse_fields = {sing: f"{sing}s" for sing in ["name", "extension"]}
        check_conflicts(collapse, [*collapse_fields.keys(), *collapse_fields.values()])

        split_fields = collapse_fields
        collapses = break_plurals(collapse, split_fields)
        if len(collapses) > 1:
            del entry["collapse"]
            entry["collapses"] = collapses

    id_fields = {sing: f"{sing}s" for sing in ["name", "pattern", "extension"]}
    check_conflicts(entry, [*id_fields.keys(), *id_fields.values()])

    split_fields = {**id_fields, "collapse": "collapses"}
    specs = break_plurals(entry, split_fields)

    return specs


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
