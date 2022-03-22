#!/usr/bin/env python3
from pls.data.utils import internal_yml_path


def init(args=None):
    """
    Initialise module variables.
    """

    from pls.globals import state

    state.state = state.State()
    state.state.parse_args(args)
    state.state.setup()

    from pls.globals import console

    console.console = console.get_console()

    from pls.config import icons, specs
    from pls.config.files import find_configs

    conf_files = find_configs()
    icons.nerd_icons, icons.emoji_icons = icons.get_icons(
        [
            internal_yml_path("nerd_icons.yml"),
            internal_yml_path("emoji_icons.yml"),
            *conf_files,
        ]
    )
    specs.node_specs = specs.get_specs(
        [
            internal_yml_path("node_specs.yml"),
            *conf_files,
        ]
    )


def main() -> None:
    """
    Represents the starting point of the application. This function:

    - accepts no inputs: options are read from CLI arguments using ``argparse``
    - returns no outputs: output is written to ``STDOUT`` using ``rich``
    """

    init(None)  # ``None`` makes ``argparse`` read real CLI args from ``sys.argv``

    from pls.config import specs
    from pls.fs.list import read_input
    from pls.globals import state
    from pls.output.table import write_output

    node_map, node_list = read_input()

    if not node_list:
        return

    for node in node_list:
        node.match_specs(specs.node_specs)
        if state.state.collapse:
            node.find_main(node_map)
    if state.state.collapse:
        for node in node_list:
            if node.is_sub:
                continue
            node.set_sub_pre_shapes()

    write_output(node_list)


if __name__ == "__main__":
    main()
