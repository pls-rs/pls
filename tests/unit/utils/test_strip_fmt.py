import pytest
from rich.markup import escape

from pls.utils.strip_fmt import strip_formatting


@pytest.mark.parametrize("should_escape", [True, False])
@pytest.mark.parametrize(
    "in_text, out_text",
    [
        ("a [b]c[/b] d", "a c d"),
        ("a []b[/]", "a []b"),  # [] is not considered Rich markup
        ("a [b c]d[/b c]", "a d"),
    ],
)
def test_strips_formatting_tags(in_text: str, out_text: str, should_escape: bool):
    if should_escape:
        in_text = escape(in_text)
        out_text = in_text
    assert strip_formatting(in_text) == out_text
