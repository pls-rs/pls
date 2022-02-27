---
lang: en-GB
title: Comparison
description: >-
  There were many reasons for developing pls, even when there are so many
  alternatives to choose from.
---

# Comparison

There are lots of alternatives to `ls`. Here are some of the most popular ones.

- [`exa`](https://github.com/ogham/exa)
- [`lsd`](https://github.com/Peltoche/lsd)
- [`colorls`](https://github.com/athityakumar/colorls)
- [`ls-go`](https://github.com/acarl005/ls-go)

`pls` stands out from all of these, thanks to some very specific choices. Here
are some examples of how `pls` is a whole class apart from these tools.

## exa

exa is likely the most popular alternative to `ls`. `pls` makes some very
different choices, considering its target audience, programmers. Programmers
usually have needs above that of the average user, which means different
defaults and unique features.

- `pls` shows dotted files (with specs) by default; exa requires the `--all`
  flag.

  - `pls` never shows the current directory `.` and parent directory `..`; exa
    does when passed the `--all` flag twice.

- `pls` uses the file name, extension and programming language to determine
  [colors](../features/colors); exa uses the file type.

- `pls` has [icons](../features/icons) turned on by default and can be opted
  out; exa needs you to opt in to icons using the `--icons` flag.

- `pls` has the concept of file [importance](../features/importance) built into
  it; exa doesn't have such this feature.

  - `pls` uses the `--all` flag for adjusting the visibility cut-off for
    importance; `exa` uses it to show files with leading dots `.`.

- `pls` makes use of a very human-friendly API for
  [details](../features/details) and [sorting](../features/sorting), exa has
  separate flags for each column.

- `pls` can read `.pls.yml` files in any directory to extend its configuration;
  exa can be customised (to a small extent) via environment variables.
