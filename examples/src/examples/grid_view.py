from examples.bench import grid_bench
from examples.utils.main import write_out


def grid_view():
    with grid_bench() as bench:
        write_out("--grid=true", bench=bench, dest_name="on", env={"PLS_COLUMNS": "28"})
        write_out(bench=bench, dest_name="off", env={"PLS_COLUMNS": "28"})


def direction():
    with grid_bench() as bench:
        write_out(
            "--grid=true",
            "--down=true",
            bench=bench,
            dest_name="on",
            env={"PLS_COLUMNS": "28"},
        )
        write_out(
            "--grid=true", bench=bench, dest_name="off", env={"PLS_COLUMNS": "28"}
        )


if __name__ == "__main__":
    grid_view()
    direction()
