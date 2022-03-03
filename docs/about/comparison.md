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

## General differences

- A lot of `ls` replacements try to maintain compatibility with `ls`'s options.
  `pls` does not do that which allows it to have a way more fluent API.
- While `pls` works for everyone, it targets the tech-savvy pro users,
  specifically developers using features tailored to their needs.
- Highly customisable using simple configuration that can be checked-in with
  your code. No other tool offers this level of customisation.
- Very meticulously chosen iconography with support for Nerd Fonts, emoji and
  even no icons at all.

### Specific differences

### exa

exa is likely the most popular alternative to `ls`. Here is some ways in which
`pls` differs from exa.

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
