"""
Do not import variables from this module, always import the module directly and use its
variables with a dot ``.`` notation.
"""

from __future__ import annotations

import copy
from functools import lru_cache
from pathlib import Path
from typing import Any, Optional

from pls.data.utils import load_yml_file
from pls.exceptions import ConfigException, ConstException


class NestedDict(dict):
    """
    Extends ``dict`` to add support for deep-merge and lookup functionality.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.lookup = lru_cache(maxsize=None)(self._lookup)

    @staticmethod
    def _deep_merge(
        a: dict, b: dict, overwrite: bool = False, path: Optional[list[str]] = None
    ):
        """
        Deep-merge dict ``b`` into dict ``a``. This function mutates ``a`` as a side
        effect and therefore does not return any value. Nested lists are union-ed and
        nested dicts are recursively deep-merged.

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
                    NestedDict._deep_merge(a[key], b[key], overwrite, path + [str(key)])
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
                    raise ConstException(f"Conflict at {'.'.join([*path, str(key)])}")
            else:
                a[key] = b[key]

    def deep_merge(self, other: dict, overwrite: bool = False):
        """
        Deep-merge the given dictionary into ``self``.

        :param other: the incoming dictionary, left unchanged
        :param overwrite: whether to write the conflicting keys with value in ``other``
        """

        self._deep_merge(self, other, overwrite)

    def _lookup(self, *path: str, default: Any = "~UNSET~") -> Any:
        """
        Lookup the given path in the dictionary by traversing the fragments. This
        assumes that the dictionary contains only contains plain data types. Use
        integers for looking up values in lists. Note that a lookup resulting in
        ``None`` is considered a passing lookup and will not return the default value.

        :param path: the path to the data point inside the dictionary
        :param default: the value to return if the path is invalid or does not exist
        :return: the value at the path if found, ``default`` otherwise
        :raises: ``ConstException`` if no value exists and ``default`` is ``None``
        """

        obj = self

        for fragment in path:
            try:
                if (isinstance(obj, list)) or isinstance(obj, dict):
                    obj = obj[fragment]
                    continue
                raise ConstException
            except (KeyError, IndexError, TypeError, ConstException) as exc:
                # ``KeyError`` when ``fragment`` not in ``obj`` dict
                # ``IndexError`` when ``fragment`` not in ``obj`` list
                # ``TypeError`` when ``fragment`` not ``int`` in ``obj`` list
                # ``ConstException`` when traversal not possible
                if default != "~UNSET~":
                    return default

                path_str = ".".join([str(fragment) for fragment in path])
                raise ConstException(f"Cannot find `{path_str}` in dict.") from exc

        return obj


def get_constants(conf_paths: list[Path]) -> NestedDict:
    """
    Constants are mappings of names and values that ``pls`` treats as hardcoded values.
    Changing constants acts as a subtle way to tweak the output.

    :param conf_paths: the list of config files from which to import constants
    :return: the mapping of icon name to icon glyph
    """

    consts: NestedDict = NestedDict()

    for conf_path in reversed(conf_paths):
        # Use a copy to prevent ``load_yml_file`` cache from being polluted.
        conf = copy.deepcopy(load_yml_file(conf_path))

        consts_val = conf.get("constants", {})
        if not consts_val:
            continue
        if not isinstance(consts_val, dict):
            raise ConfigException("[italic]`constants`[/] must be a dictionary.")
        consts.deep_merge(consts_val, overwrite=True)

    return consts


constants: NestedDict
"""the mapping of constant names to constant values"""
