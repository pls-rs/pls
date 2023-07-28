import subprocess

from examples.bench import ts_bench
from examples.utils.main import write_out, copy_write_conf


def specs():
    with ts_bench() as bench:
        subprocess.run(["git", "init", str(bench.absolute())], check=True)
        write_out(bench=bench, dest_name="def")
        copy_write_conf(bench)
        write_out(bench=bench, dest_name="confd")
        write_out(bench=bench / "src", dest_name="confd_src")


if __name__ == "__main__":
    specs()
