from __future__ import annotations

import logging
from functools import lru_cache
from pathlib import Path
from typing import Any

import yaml

from pls.exceptions import ConfigException

logger = logging.getLogger(__name__)


@lru_cache(maxsize=None)
def load_yml_file(file_path: Path) -> Any:
    """
    Load the YAML file referenced by the given name.

    :param file_path: the path to the YAML file to read and parse
    :return: the parsed contents of the YAML file
    """

    logger.debug(f"Loading YAML file: {file_path}")
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
