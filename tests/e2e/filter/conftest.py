from __future__ import annotations

import shutil
from pathlib import Path
from typing import Literal

import pytest
import yaml

from tests.e2e.utils import get_workbench

scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def imp_workbench(workbench: Path):
    def file_name(idx: int) -> str:
        return str(idx) if idx >= 0 else f"_{abs(idx)}"

    workbench = get_workbench(
        ("imp", [file_name(index) for index in range(-3, 2)]), workbench
    )
    with workbench.joinpath(".pls.yml").open("w", encoding="utf-8") as conf_file:
        conf_file.write(
            yaml.dump(
                {
                    "node_specs": [
                        {"name": file_name(index), "importance": index}
                        for index in range(-3, 2)
                    ]
                }
            )
        )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def type_workbench(workbench: Path):
    workbench = get_workbench(
        ("type", ["file_a", "file_b", ("dir_a", []), ("dir_b", [])]),
        workbench,
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def pattern_workbench(workbench: Path):
    workbench = get_workbench(("pattern", ["ab", "bc", "ca"]), workbench)
    yield workbench
    shutil.rmtree(workbench)
