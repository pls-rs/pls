---
lang: en-GB
title: Icons
description: >-
  Node specs map nodes to icon names. pls then uses icon configs then map the
  icon name to the actual icon glyph to display.
---

# Icons

Icons in `pls` are abstracted one level above the raw codepoints to allow
switching the icon between Nerd Font icons and emojis.

Icons are simply mappings of icon names and the corresponding glyph.

## Nerd Font icons

The keys in this dictionary are icon names and the values are Unicode code
points corresponding to the icon. These can be obtained from the
[Nerd Fonts cheat sheet](https://www.nerdfonts.com/cheat-sheet). See the hex
code at the bottom right of your preferred icon and prefix `\u` to it.

```yml
nerd_icons:
  lock: "\uf023" # nf-fa-lock
  key: "\uf80a" # nf-mdi-key_variant
```

::: warning
YAML has weird rules for strings. To ensure everything works as expected, wrap
the unicode escape codes in double quotes `"`.
:::

::: tip
Since it's basically impossible to mentally associate a code point with the
visual, you should put the actual class name in a comment for future reference.
:::

::: tip
You can also set your IDE font to a Nerd Font, and then just copy-paste the icon
glyph directly from the Nerd Fonts site.
:::

## Emoji icons

The keys in this dictionary are icon names and the values are the actual emoji
glyph.

```yml
emoji_icons:
  lock: "🔒"
  key: "🔑"
```

Compared to Nerd Font icons, emoji icons have fewer quirks. But they aren't as
aesthetic for several reasons.

- Emojis lack color consistency as they are all inherently colorful. This also
  prevents them from being colored using the spec `color` like text.
- Technology specific icons are largely absent from the set, leaving little to
  no choice of icons and forcing use of metaphors (like 🐍 for Python).

## Reference

For reference, you can look at the
[built-in Nerd Font icons](https://github.com/dhruvkb/pls/blob/main/src/pls/data/nerd_icons.yml)
and
[built-in emoji icons](https://github.com/dhruvkb/pls/blob/main/src/pls/data/emoji_icons.yml).
