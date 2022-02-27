from __future__ import annotations

from pathlib import Path

from pls.config.files import conf_files
from pls.data.utils import internal_yml_path, load_yaml_file


def get_icons(conf_paths: list[Path]) -> tuple[dict[str, str], dict[str, str]]:
    """
    Icons are technically just string-string maps. Nerd Font icons map icon
    names to Unicode code-points containing character glyphs and emoji icons
    map icon names to emoji characters.

    :param conf_paths: the list of config files from which to import icons
    :return: the mapping of icon name to icon glyph
    """

    nerd: dict[str, str] = {}
    emoji: dict[str, str] = {}

    for conf_path in reversed(conf_paths):
        conf = load_yaml_file(conf_path)
        nerd.update(conf.get("nerd_icons", {}))
        emoji.update(conf.get("emoji_icons", {}))

    return nerd, emoji


all_icons = get_icons(
    [
        internal_yml_path("nerd_icons.yml"),
        internal_yml_path("emoji_icons.yml"),
        *conf_files,
    ]
)

nerd_icons = all_icons[0]
"""the mapping of icon names to Unicode code-points"""

emoji_icons = all_icons[1]
"""the mapping of icon names to emoji characters"""
