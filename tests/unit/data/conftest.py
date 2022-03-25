from pathlib import Path
from typing import Literal

import pytest
import yaml

from pls.data.utils import internal_yml_path


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def schema():
    schema_path = internal_yml_path("schema/pls_config.yml")
    with schema_path.open("r", encoding="utf-8") as schema_file:
        data = yaml.safe_load(schema_file)
    return data


@pytest.fixture(scope=scope)
def get_config():
    def _get_config(conf_name: str):
        conftest_path = Path(__file__)
        data = conftest_path.parents[3].joinpath("src", "pls", "data")
        return data.joinpath(conf_name)

    return _get_config
