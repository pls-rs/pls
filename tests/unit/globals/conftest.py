import shutil
from pathlib import Path
from typing import Literal
from unittest.mock import MagicMock, patch

import git
import pytest

scope: Literal["package"] = "package"


@pytest.fixture
def pwd_grp():
    mock_user = MagicMock(pw_name="usr1", pw_gid=0)
    pwd = MagicMock(getpwuid=MagicMock(return_value=mock_user))
    grp = MagicMock(
        getgrall=MagicMock(
            return_value=[
                MagicMock(gr_name="grp1", gr_gid=1, gr_mem=["usr1"]),
                MagicMock(gr_name="grp2", gr_gid=2, gr_mem=["usr2", "usr1"]),
                MagicMock(gr_name="grp3", gr_gid=3, gr_mem=["usr2"]),
            ]
        )
    )

    patchers = [
        patch.dict("sys.modules", {"pwd": pwd, "grp": grp}),
        patch("os.getuid", MagicMock(return_value=219)),
    ]
    for patcher in patchers:
        patcher.start()

    yield pwd, grp

    for patcher in patchers:
        patcher.stop()


@pytest.fixture(scope=scope)
def workbench():
    conftest_path = Path(__file__)
    workbench = conftest_path.parent.joinpath("workbench")
    workbench.mkdir(mode=0o755)

    yield workbench

    # See reason for ``ignore_errors`` below.
    shutil.rmtree(workbench, ignore_errors=True)


@pytest.fixture(scope=scope)
def git_area(workbench: Path):
    area = workbench.joinpath("git")
    area.mkdir(mode=0o755)

    def echo(text: str, target_name: str):
        target_path = area.joinpath(target_name)
        with target_path.open(mode="a") as file:
            file.write(text)

    for i in range(10):
        file_name = f"file_{i+1:02}"
        echo(file_name, file_name)

    repo = git.Repo.init(area)
    echo("file_02", ".gitignore")
    repo.index.add([f"file_{i:02}" for i in range(3, 8)])
    repo.index.commit("Add files")
    echo(" ", "file_04")
    echo(" ", "file_05")
    repo.index.add(["file_05"])
    echo(" ", "file_05")
    repo.index.move(["file_06", "file_76"])
    area.joinpath("file_07").unlink()
    repo.index.add(["file_08"])
    repo.index.add(["file_09"])
    echo(" ", "file_09")
    repo.index.add(["file_10"])
    area.joinpath("file_10").unlink()

    yield area

    # This raises errors on Windows in CI due to the directory being in use.
    #
    # Python error:
    # PermissionError: [WinError 32] The process cannot access the file because it is
    #   being used by another process:
    #
    # Bash error:
    # rm: cannot remove 'git': Device or resource busy
    shutil.rmtree(area, ignore_errors=True)
