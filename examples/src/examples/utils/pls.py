from decouple import config

from examples.utils.sub import run_cmd


def run_pls(args: list[str], **kwargs) -> str:
    """
    Run a ``pls`` command and return the output with ANSI codes.

    It assumes the project root to be the working directory and that a release
    build of ``pls`` is present on the ``$PATH``.

    All keyword arguments are forwarded as-is to ``run_cmd``.

    :return: the output of the ``pls`` command
    """

    pls_bin = config("PLS_BIN", default="pls")
    cmd = [pls_bin, *args]
    print(f"Running command {cmd}")

    env = kwargs.pop("env", {})
    if "NO_COLOR" not in env:
        env["CLICOLOR_FORCE"] = "true"
    proc = run_cmd(cmd, env=env, **kwargs)
    return f'```ansi frame="none"\n{proc.stdout}```\n'
