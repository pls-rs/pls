import re


def strip_formatting(fmt_text: str) -> str:
    """
    Strip all Rich formatting tags out of the given string, returning only the
    underlying plain text.

    :param fmt_text: the formatted text from which to strip tags
    :return: the text stripped of all formatting
    """

    fmt_pattern = re.compile(r"(?<!\\)\[([a-z0-9_\s]+|/[a-z0-9_\s]*)]")
    return re.sub(fmt_pattern, "", fmt_text)
