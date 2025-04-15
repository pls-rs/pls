from pathlib import Path

DOCS_EXAMPLES = Path(__file__).parents[4] / "docs" / "src" / "examples"


def write_content(dest_path: str, content: str):
    """
    Write the given content to a file with the given path.

    The path is joined with the docs' ``examples`` directory. If the path
    contains a directory name which does not exist, it will be created.

    :param dest_path: the additional path of the output file
    :param content: the content to write inside the file
    """

    dest_path = DOCS_EXAMPLES / dest_path
    if not dest_path.parent.exists():
        dest_path.parent.mkdir(mode=0o755, parents=True)

    print(f"MDX file written to '{dest_path}'.")
    dest_path.write_text(content, encoding="utf-8")
