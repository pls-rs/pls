from pathlib import Path
from typing import Callable

import pytest
import yaml
from jsonschema import validate


@pytest.mark.parametrize(
    "config_name",
    ["emoji_icons.yml", "nerd_icons.yml", "node_specs.yml"],
)
def test_config_is_valid(
    config_name: str, get_config: Callable[[str], Path], schema: dict
):
    config = get_config(config_name)
    instance = yaml.safe_load(config.open("r", encoding="utf-8"))
    validate(instance, schema)
