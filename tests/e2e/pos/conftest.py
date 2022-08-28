from __future__ import annotations

import shutil
from pathlib import Path
from typing import Literal

import pytest

from tests.e2e.utils import get_workbench


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def pos_workbenches(workbench: Path):
    workbench_a = get_workbench(
        ("a", [f"a_{index}" for index in range(1, 3)]), workbench
    )
    workbench_b = get_workbench(
        ("b", [f"b_{index}" for index in range(1, 3)]), workbench
    )
    yield workbench_a, workbench_b
    shutil.rmtree(workbench_a)
    shutil.rmtree(workbench_b)
