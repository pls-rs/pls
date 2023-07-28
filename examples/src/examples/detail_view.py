from examples.bench import typ_bench
from examples.utils.fs import fs, mkbigfile
from examples.utils.main import write_out, copy_write_conf


def detail_view():
    with typ_bench() as bench:
        write_out("--det=all", bench=bench, dest_name="all")
        write_out("--det=user", "--det=group", bench=bench, dest_name="sel")
        write_out("--det=std", bench=bench, dest_name="std")
        write_out("--det=none", bench=bench, dest_name="none")
        copy_write_conf(bench)
        write_out("--det=all", bench=bench, dest_name="confd")


def header():
    with typ_bench() as bench:
        write_out("--det=std", bench=bench, dest_name="on")
        write_out("--det=std", "--header=false", bench=bench, dest_name="off")


def units():
    with fs(
        (
            "units",
            [
                (name, lambda p: mkbigfile(p, size=1024**idx))
                for (idx, name) in enumerate("abc")
            ],
        )
    ) as bench:
        for unit in ["binary", "decimal", "none"]:
            write_out("--det=size", f"--unit={unit}", bench=bench, dest_name=unit)


if __name__ == "__main__":
    detail_view()
    header()
    units()
