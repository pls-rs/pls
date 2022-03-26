from __future__ import annotations

from pathlib import Path
from typing import Any, Optional

import yaml

from pls.exceptions import ConfigException, ConstException


def deep_merge(
    a: dict, b: dict, overwrite: bool = False, path: Optional[list[str]] = None
):
    """
    Deep merge nested dictionaries. This function merges dictionary ``b`` into ``a``,
    using recursion to merge any nested dictionaries with the same key. This function
    mutates ``a`` as a side effect and therefore does not return any value.

    :param a: the first dictionary, mutated in the merge
    :param b: the second dictionary, left unchanged
    :param overwrite: whether to write the conflicting keys with value in ``b``
    :param path: a parameter to track the location of the conflict
    """

    if path is None:
        path = []
    for key in b:
        if key in a:
            if isinstance(a[key], dict) and isinstance(b[key], dict):
                # Deep merge nested dictionaries.
                deep_merge(a[key], b[key], overwrite, path + [str(key)])
            elif isinstance(a[key], list) and isinstance(b[key], list):
                # Union lists.
                a[key].extend(b[key])
            elif a[key] == b[key]:
                # Ignore as both values are equal.
                pass
            elif overwrite:
                # Overwrite keys that cannot be merged.
                a[key] = b[key]
            else:
                # Cannot merge incompatible data types.
                raise ValueError(f"Conflict at {'.'.join([*path, str(key)])}")
        else:
            a[key] = b[key]


def lookup(obj: dict, path: list[str], default: Any = None) -> Any:
    """
    Lookup the given path in the dictionary by traversing the fragments. This
    assumes that the dictionary contains only contains plain data types. Use
    integers for looking up values in lists.

    :param obj: the dictionary in which to search for the path
    :param path: the path to the data point inside the dictionary
    :param default: the value to return if the path is invalid or does not exist
    :return: the value at the path if found, ``default`` otherwise
    :raises: ``ConstException`` if no value exists and ``default`` is ``None``
    """

    for fragment in path:
        try:
            if (isinstance(obj, list)) or isinstance(obj, dict):
                obj = obj[fragment]
                continue
            else:
                raise ConstException
        except (KeyError, IndexError, TypeError, ConstException) as exc:
            # ``KeyError`` when ``fragment`` not in ``obj`` dict
            # ``IndexError`` when ``fragment`` not in ``obj`` list
            # ``TypeError`` when ``fragment`` not ``int`` in ``obj`` list
            # ``ConstException`` when traversal not possible
            if default is not None:
                return default

            path_str = ".".join([str(fragment) for fragment in path])
            raise ConstException(f"Cannot find `{path_str}` in dict.") from exc

    return obj


def load_yml_file(file_path: Path) -> Any:
    """
    Load the YAML file referenced by the given name.

    :param file_path: the path to the YAML file to read and parse
    :return: the parsed contents of the YAML file
    """

    try:
        with file_path.open("r", encoding="utf-8") as data_file:
            data = yaml.safe_load(data_file)
        return data
    except yaml.YAMLError as exc:
        raise ConfigException(f"{file_path} is not valid YAML.") from exc


def internal_yml_path(file_name: str) -> Path:
    """
    Map the given YAML data file name to the path, assuming that all the data
    files are in the same directory as the file of this function.

    :param file_name: the name of the YAML data file
    :return: the path to the YAML data file
    """

    return Path(__file__).parent.joinpath(file_name)
