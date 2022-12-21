from __future__ import annotations

from functools import cached_property

TEXT_ONLY_RULES = {"bold", "italic", "underline", "strike"}


class FormatRules(list):
    """
    Represents two types of formatting rules, for icons and text. This class extends the
    built-in ``list`` type, so you can use methods such as ``append`` and ``extend``.
    """

    @staticmethod
    def get_format_pair(rules: list[str]) -> tuple[str, str]:
        """
        Get a two element tuple containing the opening and closing tags of Rich console
        formatting markup.

        :param rules: the rules to convert to console markup
        :return: the pair of opening and closing formatting tags
        """

        if rules:
            left = f"[{' '.join(rules)}]"
            right = "[/]"
        else:
            left = right = ""
        return left, right

    @cached_property
    def icon_fmt_rules(self) -> list[str]:
        return [rule for rule in self if rule not in TEXT_ONLY_RULES]

    @cached_property
    def icon_format_pair(self) -> tuple[str, str]:
        return self.get_format_pair(self.icon_fmt_rules)

    def format_icon(self, icon: str) -> str:
        left, right = self.icon_format_pair
        return f"{left}{icon}{right}"

    @cached_property
    def text_format_pair(self) -> tuple[str, str]:
        return self.get_format_pair(self)

    def format_text(self, text: str) -> str:
        left, right = self.text_format_pair
        return f"{left}{text}{right}"
