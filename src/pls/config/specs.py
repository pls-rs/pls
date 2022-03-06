from __future__ import annotations

from pathlib import Path

from pls.config.files import conf_files
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

    specs = break_plurals(entry, id_fields)
    specs = [
        spec
        for entry in specs
        for spec in break_plurals(entry, {"collapse": "collapses"})
    ]

    return specs


def get_specs(conf_paths: list[Path]) -> list[NodeSpec]:
    """
    Parse information about the list of all node specs for all languages. This
    maps the POPO representation of the specs into the ``NodeSpec`` instances.

    :param conf_paths: the list of config files from which to import spec POPOs
    :return: the list of all node specs for all languages
    """

    entries = []

    for conf_path in conf_paths:
        conf = load_yaml_file(conf_path)
        entries.extend(conf.get("node_specs", []))

    return [NodeSpec(**spec) for entry in entries for spec in massage_specs(entry)]


node_specs = get_specs(
    [
        internal_yml_path("node_specs.yml"),
        *conf_files,
    ]
)
"""the list of all node specs for all types of nodes"""
