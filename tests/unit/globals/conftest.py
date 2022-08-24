import shutil
import subprocess
from pathlib import Path
from typing import Literal
from unittest.mock import MagicMock, patch

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

    shutil.rmtree(workbench)


@pytest.fixture(scope=scope)
def git_area(workbench: Path):
    area = workbench.joinpath("git")
    area.mkdir(mode=0o755)

    for i in range(10):
        file_name = f"file_{i+1:02}"
        file_path = area.joinpath(file_name)
        with file_path.open(mode="w") as file:
            file.write(file_name)

    git_cmds = f"""
        git init
        echo "file_02" >> .gitignore
        git add {' '.join(f'file_{i:02}' for i in range(3,8))}
        git commit -m "add files"
        echo " " >> file_04
        echo " " >> file_05; git add file_05; echo " " >> file_05
        git mv file_06 file_76
        rm file_07
        git add file_08
        git add file_09; echo " " >> file_09
        git add file_10; rm file_10
    """
    subprocess.run(
        git_cmds,
        shell=True,
        check=True,
        cwd=area,
        text=True,
        encoding="utf-8",
    )

    yield area

    shutil.rmtree(area)
