import shutil
from pathlib import Path

import pytest


@pytest.fixture
def work_dirs():
    conftest_path = Path(__file__)
    workbench = conftest_path.parent.joinpath("workbench")

    # Make dirs and files
    one = workbench.joinpath("one")
    two = one.joinpath("two")
    three = two.joinpath("three")

    three.mkdir(parents=True, mode=0o755)

    for dir in [one, two, three]:
        dir.joinpath(".pls.yml").touch(mode=0o644)

    yield workbench, one, two, three

    shutil.rmtree(workbench)
