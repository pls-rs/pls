from __future__ import annotations

import copy
import shutil
from pathlib import Path
from typing import Literal

import pytest
import yaml

from pls.config.constants import NestedDict


scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def workbench():
    conftest_path = Path(__file__)
    workbench = conftest_path.parent.joinpath("workbench")
    workbench.mkdir(mode=0o755)

    yield workbench

    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def work_dirs(workbench: Path):
    # Make dirs and files
    one = workbench.joinpath("one")
    two = one.joinpath("two")
    three = two.joinpath("three")

    three.mkdir(parents=True, mode=0o755)

    yield one, two, three


@pytest.fixture(scope=scope)
def get_conf():
    def _get_conf(path: Path):
        # region hardcoded configs
        configs: dict[str, dict] = {
            "three": {
                "node_specs": [
                    {
                        "name": "cat.py",
                        "icon": "cat",
                    },
                ],
                "nerd_icons": {
                    "cat": "\ufbd9",  # ﯙ
                    "dog": "\ue251",  # 
                },
                "emoji_icons": {
                    "cat": "🐈",
                    "dog": "🐶",
                },
                "prefs": {
                    "sort": "name",
                    "dirs_first": True,
                },
                "constants": {
                    "type_chars": {
                        "symlink": "\uf838",  # 
                        "fifo": "p",
                    },
                },
            },
            "two": {
                "node_specs": [
                    {"extension": "py", "icon": "mouse", "importance": 1},
                ],
                "nerd_icons": {
                    "mouse": "\uf87c",  # 
                    "cat": "\uf61a",  # 
                },
                "emoji_icons": {
                    "mouse": "🐭",
                    "cat": "🐱",
                },
                "prefs": {
                    "dirs_first": False,
                    "time_fmt": "[dim]%Y-[/]%m-%d %H:%M[dim]:%S[/] ",
                },
                "constants": {
                    "type_chars": {
                        "symlink": "l",
                        "dir": "d",
                    },
                },
            },
        }
        # endregion
        conf_path = path.joinpath(".pls.yml")
        conf_data = configs.get(path.name, {})
        with conf_path.open("w", encoding="utf-8") as conf_file:
            yaml.dump(conf_data, conf_file)
        return path.joinpath(".pls.yml")

    return _get_conf


@pytest.fixture
def nested_dict() -> NestedDict:
    regular_dict = copy.deepcopy(
        {
            "dict": {"a": 1},
            "list": ["a"],
            "scalar": "val",
        }
    )
    return NestedDict(regular_dict)
