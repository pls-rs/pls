from pls.models.format_rules import FormatRules


def test_format_rules_extends_list():
    format_rules = FormatRules()
    assert isinstance(format_rules, list)


def test_formats_text_with_rules():
    format_rules = FormatRules(["bold", "italic", "underline", "blue"])
    output = format_rules.format_text("Hello, World!")
    assert output == "[bold italic underline blue]Hello, World![/]"


def test_formats_text_without_rules():
    format_rules = FormatRules()
    output = format_rules.format_text("Hello, World!")
    assert output == "Hello, World!"


def test_formats_icon_with_rules():
    format_rules = FormatRules(["dim", "blue", "on red"])
    output = format_rules.format_icon("X")
    assert output == "[dim blue on red]X[/]"


def test_formats_icon_without_rules():
    format_rules = FormatRules()
    output = format_rules.format_icon("X")
    assert output == "X"


def test_formats_icon_except_text_only_rules():
    format_rules = FormatRules(["dim", "blue", "on red", "bold", "italic", "underline"])
    output = format_rules.format_icon("X")
    assert output == "[dim blue on red]X[/]"
