---
lang: en-GB
title: Installation
description: >-
  pls is a breeze to install and update. Provided you have the prerequisites,
  you'll be ready in no time.
---

# Installation

Thanks for giving `pls` a try. `pls`, being a Python package, needs Python 3.8
or newer. There are two approaches to installing `pls`, using either `pipx`
(which is a great tool for managing Python binaries) or `pip` (which comes
pre-installed with Python).

## Requirements

To install `pls`, you need to have the following software installed.

- Python 3.8
- `pip` or `pipx` (your preferred mode of installation)

Some features need additional dependencies.

- [Icons need Nerd Fonts](../features/icons.md#requirements)
- [Git status column needs Git](../features/details.md#git-status-git)

## With `pipx`

[`pipx`](https://pypa.github.io/pipx/) is a fine way to install executable
Python packages globally while still isolating them in virtual environments.
This allows for clean upgrades and uninstalls. You can learn more about `pipx`
from their documentation.

For new installs, we recommend using this approach.

### Prerequisites

To install `pls` using `pipx` you need to first install `pipx`! Read the `pipx`
documentation to [learn how](https://pypa.github.io/pipx/installation/).

### Installing

```shellsession
$ pipx install pls
```

### Updating

```shellsession
$ pipx upgrade pls
```

### Uninstalling

```shellsession
$ pipx uninstall pls
```

## With `pip`

This is the simplest way to install and use `pls`. It does have one major
drawback: it installs lots of packages to the global namespace that will be left
behind after `pls` is uninstalled.

We recommend [giving `pipx` a try](#with-pipx).

### Prerequisites

To install using `pip`, you need to have `pip`. If you have Python installed, it
is very likely you also have `pip` installed too.

### Installing

```shellsession
$ pip install --user pls
```

### Updating

```shellsession
$ pip install --user --upgrade pls
```

### Uninstalling

```shellsession
$ pip uninstall pls
```

## Verifying

To check if `pls` is installed and discoverable in your path, run the following
command. If the version number you see is the same as the latest version on
[PyPI](https://pypi.org/project/pls/), you're all set!

```shellsession
$ pls --version
$ pls -v
```

```
pls x.y.z
```
