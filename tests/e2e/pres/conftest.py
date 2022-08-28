from __future__ import annotations

import shutil
from pathlib import Path
from typing import Literal

import pytest

from tests.e2e.utils import get_workbench


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def icon_workbench(workbench: Path):
    workbench = get_workbench(
        (
            "icon",
            [
                ".gitignore",  # matched by name
                "docker-compose.yml",  # matched by pattern
                "README.md",  # matched by extension
            ],
        ),
        workbench,
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def align_workbench(workbench: Path):
    workbench = get_workbench(
        (
            "align",
            [
                ".gitignore",  # has a leading dot
                "README.md",  # does not have a leading dot
            ],
        ),
        workbench,
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def multi_cols_workbench(workbench: Path):
    workbench = get_workbench(("multi_cols", ["a", "b", "c"]), workbench)
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def collapse_workbench(workbench: Path):
    workbench = get_workbench(("collapse", ["style.scss", "style.css"]), workbench)
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def tree_workbench(workbench: Path):
    workbench = get_workbench(("tree", [("a", ["b", ("c", ["d"])]), "e"]), workbench)
    yield workbench
    shutil.rmtree(workbench)
