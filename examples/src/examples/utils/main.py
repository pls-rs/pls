import inspect
import shutil
from pathlib import Path

from examples.utils.to_html import run_to_html
from examples.utils.io import write_content


CONFS = Path(__file__).parents[1] / "confs"


def write_out(
    *args: str,
    bench: Path | None = None,
    include_bench: bool = True,
    dest_name: str,
    **kwargs,
):
    """
    Run ``pls`` with the given arguments and write the output to an MDX file.

    :param args: the arguments to pass to ``pls`` (except the workbench path)
    :param bench: the path to the workbench
    :param include_bench: whether to include the workbench path in the arguments
    :param dest_name: the name of the output MDX file
    :param kwargs: addition keyword arguments to pass to ``run_pls``
    """

    func_name = _caller()

    args = list(args)
    if bench and include_bench:
        args.append(str(bench.absolute()))
    content = run_to_html(args, **kwargs)

    out_file = f"{func_name}/{dest_name}.mdx"
    print(f"Writing MDX to '{out_file}'.")
    write_content(out_file, content)


def copy_write_conf(path: Path, name: str | None = None):
    """
    Move the config file from the ``confs`` directory into the workbench and
    write a copy as MDX.

    This function also exports the config as an MDX file in the ``examples``
    directory of the docs.

    :param path: the path to the workbench
    :param name: the name of the source and destination config file
    """

    func_name = _caller()

    src = CONFS / f"{name or func_name}.yml"
    dest = path / ".pls.yml"
    print(f"Copying '{src}' to '{dest}'.")
    shutil.copy(src, dest)

    out_file = f"{func_name}/{name or 'conf'}.mdx"
    print(f"MDX destination is '{out_file}'.")
    write_content(out_file, f"```yml\n{src.read_text()}```")


def _caller() -> str:
    """
    Get the name of the file two levels above the current frame. This function
    only returns the name, stripping away the path and the extension.

    :return: the name of the file
    """

    prev_frame = inspect.currentframe().f_back.f_back
    func_name = inspect.getframeinfo(prev_frame).function
    return func_name
