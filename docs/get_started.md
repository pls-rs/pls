---
lang: en-GB
title: Get started
description: >-
  Installing pls is as easy as 'pip install --user pls', provided you already
  have the prerequisites.
---

# Get started

Thanks for giving `pls` a try. `pls`, being a Python package, needs Python 3.8
or newer.

## With `pip`

This is the simplest way to install and use `pls`. It does have one major
drawback: it installs lots of packages to the global namespace that will be left
behind after `pls` is uninstalled.

We recommend [giving `pipx` a try](#with-pipx).

### Prerequisites

To install using `pip`, you need to have `pip`. If you have Python installed, it
is very likely you also have `pip` installed too.

### Installing

```
$ pip install --user pls
```

### Updating

```
$ pip install --user --upgrade pls
```

### Uninstalling

```
$ pip uninstall pls
```

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

```
$ pipx install pls
```

### Updating

```
$ pipx upgrade pls
```

### Uninstalling

```
$ pipx uninstall poetry
```

## Verifying

To check if `pls` is installed and discoverable in your path, run the following
command. If you see the same version number as on
[PyPI](https://pypi.org/project/pls/), you're all set!

```
$ pls -v
pls x.y.z
```

## Using

To run `pls`, type the command into any terminal.

```
$ pls
```

To get help, run `pls` with the `--help`/`-h` flag or _read this documentation_!

```
$ pls --help
```
