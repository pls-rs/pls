from __future__ import annotations

import shutil
import subprocess
from pathlib import Path
from typing import Literal

import pytest


scope: Literal["package"] = "package"


def get_workbench(parent: Path, name: str, file_names: list[str] = None) -> Path:
    """
    Create a workbench with the given name inside the given directory. If a list of file
    names are provided, they will be created as blank files using ``touch``.

    :param parent: the path to the parent directory inside which to create the workbench
    :param name: the name of the workbench directory
    :param file_names: the name of files to create inside the workbench
    :return: the newly created workbench directory
    """

    workbench = parent.joinpath(name)
    workbench.mkdir(mode=0o755)

    if file_names is None:
        file_names = []
    for file_name in file_names:
        workbench.joinpath(file_name).touch(mode=0o644)

    return workbench


@pytest.fixture(scope=scope)
def workbench():
    conftest_path = Path(__file__)
    workbench = get_workbench(conftest_path.parent, "workbench")

    # Prevents use of config files outside workbench
    subprocess.run(["git", "init"], cwd=workbench)

    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def icon_workbench(workbench: Path):
    workbench = get_workbench(
        workbench,
        "icon",
        [
            ".gitignore",  # matched by name
            "docker-compose.yml",  # matched by pattern
            "README.md",  # matched by extension
        ],
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def align_workbench(workbench: Path):
    workbench = get_workbench(
        workbench,
        "align",
        [
            ".gitignore",  # has a leading dot
            "README.md",  # does not have a leading dot
        ],
    )
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def multi_cols_workbench(workbench: Path):
    workbench = get_workbench(workbench, "multi_cols", ["a", "b", "c"])
    yield workbench
    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def collapse_workbench(workbench: Path):
    workbench = get_workbench(workbench, "collapse", ["style.scss", "style.css"])
    yield workbench
    shutil.rmtree(workbench)
