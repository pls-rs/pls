"""
Do not import variables from this module, always import the module directly and use its
variables with a dot ``.`` notation.
"""

from __future__ import annotations

import copy
import logging
from pathlib import Path

from pls.data.utils import load_yml_file
from pls.exceptions import SpecException
from pls.models.node_spec import NodeSpec


logger = logging.getLogger(__name__)


def break_plurals(entry: dict, keys: list[str]) -> list[dict]:
    """
    Split a dict with a key containing a list into separate dictionaries, with
    the singular form of the key containing one item of the list.

    The function checks against:

    - singular keys containing a list
    - plural keys not containing a list

    :param entry: the dict to split from plural to singular
    :param keys: the list of keys in which to split plural values
    :return: a list of dictionaries from splitting the plurals
    :raise: ``ConfigException``, if the fields have the wrong type of value
    """

    for key in keys:
        value = entry.get(key)
        if isinstance(value, list):
            common = {k: v for k, v in entry.items() if k != key}
            return [{key: item, **common} for item in value]

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
        fields = ", ".join([f"[italic]`{field}`[/]" for field in conflict_keys])
        raise SpecException(f"Exactly one of {fields} is allowed.", fail_spec=entry)


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
    logger.debug(f"Massaging {entry}")
    if (collapse := entry.get("collapse")) and not entry.get("collapse_fixed"):
        collapse_fields = ["name", "extension"]
        check_conflicts(collapse, collapse_fields)
        collapses = break_plurals(collapse, collapse_fields)

        if len(collapses) > 1:
            entry["collapse"] = collapses

        entry["collapse_fixed"] = True  # Prevent massaging the same spec again.

    id_fields = ["name", "pattern", "extension"]
    check_conflicts(entry, id_fields)
    specs = break_plurals(entry, id_fields)

    specs = [spec for entry in specs for spec in break_plurals(entry, ["collapse"])]

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
        # Use a copy to prevent ``load_yml_file`` cache from being polluted.
        conf = copy.deepcopy(load_yml_file(conf_path))
        entries.extend(conf.get("node_specs", []))

    return [NodeSpec(**spec) for entry in entries for spec in massage_specs(entry)]


node_specs: list[NodeSpec]
"""the list of all node specs for all types of nodes"""
