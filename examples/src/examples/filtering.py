from examples.bench import typ_bench
from examples.utils.main import write_out, copy_write_conf
from examples.utils.fs import fs


def type_filter():
    with typ_bench() as bench:
        write_out(bench=bench, dest_name="off")
        write_out("--typ=dir", "--typ=symlink", bench=bench, dest_name="on")
        write_out("--typ=dir", "fifo", cwd=bench, include_bench=False, dest_name="dis")


def name_filter():
    with fs(
        (
            "name_filter",
            ["a.rs", "b.rs", "c.rs", "a.jpg", "b.jpg", "c.jpeg"],
        )
    ) as bench:
        write_out("--only='(a|c)'", r"--exclude='\.jpe?g'", bench=bench, dest_name="on")
        write_out(bench=bench, dest_name="off")
        write_out(
            r"--exclude='\.jpe?g'",
            "a.jpg",
            cwd=bench,
            include_bench=False,
            dest_name="dis",
        )


def importance():
    with fs(
        (
            "importance",
            [
                (".git", []),  # importance -2
                (".github", []),  # importance -1
                ("dir", []),  # importance 0
                "file",  # importance 0
                "src",  # importance 1
                "README.md",  # importance 2
            ],
        )
    ) as bench:
        for imp in range(-2, 3):
            write_out(
                f"--imp={imp}", bench=bench, dest_name=f"imp_{imp}".replace("-", "m")
            )
        write_out("--imp=2", "file", cwd=bench, include_bench=False, dest_name="dis")
        copy_write_conf(bench)
        write_out(bench=bench, dest_name="confd")


if __name__ == "__main__":
    name_filter()
    type_filter()
    importance()
