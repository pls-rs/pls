---
lang: en-GB
title: Node spec
description: >-
  The code of pls functionality are specs. Specs describe the nature and purpose
  of files and folders and configure how pls renders them.
---

# Node spec

A node spec determines how each node is supposed to be displayed. As such, each
spec consists of two components.

- identification method
- rendering configuration

## Identification method

A node can be identified based on the following parameters:

- name
- extension
- regular expression pattern

### Name

This is a string that is compared for equality against the full name of a node,
including the extension if any.

### Extension

This is a string that is compared against the node's extension, which is defined
as the last segment when the file name is split at the dot `.` characters. If
the name contains no dots, it does not have an extension.

### Pattern

This is a string containing a regular expression compared against the entire
file name, including the extension if any.

The file is considered to be a match if the regex matches the file name from the
start. This is the most versatile of all identification approaches.

::: warning
The pattern here is a regular expression and not a glob pattern. Also since the
regex is matched from the start of the string, the leading caret `^` is
optional. However, to match the end of the string, a trailing dollar sign `$`
is required.
:::

## Rendering configuration

The rendering configuration specifies the following attributes.

- icon
- importance
- color
- collapse

### Icon

The icon attributes sets the name of the icon that will be associated with the
matching nodes. This is not the actual icon code point or emoji, just the name
for the icon. You can omit this field to not show any icon for the node.

See [feature docs](../features/icons).

::: warning
You will also need to configure the `emoji_icons` and `nerd_icons` for the icons
to actually show up in the output.
:::

### Importance

This can be an integer, positive or negative, specifying the relevance of the
file or folder to your workflow. You can omit this field to use the default
value of 0.

See [feature docs](../features/importance).

### Color

You can specify the color in one of three forms.

- [color name](https://rich.readthedocs.io/en/stable/appendix/colors.html)
- hex code, between `#000000` and `#ffffff`
- RGB color code, between `rgb(0, 0, 0)` and `rgb(255, 255, 255)`

::: tip
You can use the entire 256<sup>3</sup> range of colors in the spec. They will
automatically be mapped to the nearest terminal-safe color.
:::

See [feature docs](../features/colors).

#### Examples

```yml{3}
patterns: [docker-compose\b, Dockerfile\b]
icon: docker
color: "#2496ed"
```

`Dockerfile`s and Docker Compose config files are rendered in Docker's blue
color <ColorPreview color="#2496ed"/>.

### Collapse

You can collapse files behind other files.

- Specs that use `name`/`names` as the identification method can use
  `name`/`names` as the collapse identification method.
- Specs that use `extension`/`extensions` as the identification method can use
  `extension`/`extensions` as the collapse identification method.
- Collapsing is not enabled for pattern-based file identification at the moment.

See [feature docs](../features/collapse).

#### Examples

```yml{2-3}
name: Pipfile.lock
collapse:
  name: Pipfile
```

This collapses `Pipfile.lock` behind `Pipfile`. If there is no `Pipfile`, there
will be no collapse.

```yml{4-5}
extension: css
icon: css
color: "#1572b6"
collapse:
  extension: [scss, sass, less]
```

This collapses all files with a `.css` extension behind files with the same name
and each extension from the list, in order, till a match is found.

So if the folder contains the files `style.css`, `style.sass` and `style.less`,
the CSS file will be collapsed behind the SASS file.

## Shorthand

Each of the three identification methods can take multiple values as an array.
These are expanded into separate specs when loading.

For example, consider a spec like the following.

```yml
- name: [a, b, c]
```

This, upon loading, will be expanded to three specs.

```yml
- name: a
- name: b
- name: c
```

The same also happens for `pattern` and `extension`.

Similarly, when defining collapse rules, you can set `name` and `extension` to
arrays, similar to the shorthand notation described [above](#shorthand).

For example, consider a spec like the following.

```yml
- name: [a, b]
  collapse:
    - name: [c, d]
```

This, upon loading, will be expanded to four specs, in this specific order
because the top-level `name` is split first, followed by `collapse.name`.

```yml
- name: a
  collapse:
    name: c
- name: a
  collapse:
    name: d
- name: b
  collapse:
    name: c
- name: b
  collapse:
    name: d
```

The same also happens for `extension`.

## Reference

For reference, you can look at the
[built-in node specs](https://github.com/dhruvkb/pls/blob/main/src/pls/data/node_specs.yml)
from `pls`.
