from pathlib import Path
from typing import Literal

import pytest
import requests


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def schema():
    schema_url = "https://dhruvkb.github.io/pls/schemas/pls_config.json"
    response = requests.get(schema_url)
    assert response.status_code == 200
    return response.json()


@pytest.fixture(scope=scope)
def get_config():
    def _get_config(conf_name: str):
        conftest_path = Path(__file__)
        data = conftest_path.parents[3].joinpath("src", "pls", "data")
        return data.joinpath(conf_name)

    return _get_config
