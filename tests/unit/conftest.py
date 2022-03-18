from unittest.mock import MagicMock, patch

import pytest


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
