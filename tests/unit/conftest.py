from typing import Literal

import pytest


scope: Literal["session"] = "session"


@pytest.fixture(autouse=True, scope=scope)
def init():
    from pls.main import general_init, node_specific_init

    cli_args = general_init([])  # initialise with blank arguments
    if len(cli_args.nodes) == 1:
        [workdir] = cli_args.nodes
        node_specific_init(workdir, cli_args)
