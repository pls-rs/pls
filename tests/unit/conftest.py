from typing import Literal

import pytest


scope: Literal["session"] = "session"


@pytest.fixture(autouse=True, scope=scope)
def init():
    from pls.main import init

    init([])  # initialise with blank arguments
