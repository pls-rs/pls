from __future__ import annotations

import shutil
from pathlib import Path
from typing import Literal

import pytest

from tests.e2e.utils import get_workbench


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def type_workbench(workbench: Path):
    workbench = get_workbench(
        "type",
        workbench,
        [
            "file_a",
            "file_b",
            lambda par: par.joinpath("dir_a").mkdir(mode=0o755),
            lambda par: par.joinpath("dir_b").mkdir(mode=0o755),
        ],
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def pattern_workbench(workbench: Path):
    workbench = get_workbench("pattern", workbench, ["ab", "bc", "ca"])
    yield workbench
    shutil.rmtree(workbench)
