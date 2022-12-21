import shutil
import subprocess
from pathlib import Path
from typing import Literal

import pytest

from tests.e2e.utils import get_workbench

scope: Literal["package"] = "package"


@pytest.fixture(scope=scope)
def workbench():
    conftest_path = Path(__file__)
    workbench = get_workbench(("workbench", []), conftest_path.parent)

    # Prevents use of config files outside workbench
    subprocess.run(["git", "init"], cwd=workbench)

    yield workbench
    shutil.rmtree(workbench)
