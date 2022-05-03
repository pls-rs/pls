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
        "imp", workbench, [file_name(index) for index in range(-3, 2)]
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
