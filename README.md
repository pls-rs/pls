<h1 align="center">
  <img height="128px" src="https://raw.githubusercontent.com/dhruvkb/pls/main/readme_assets/pls.svg"/>
</h1>

<p align="center">
  <a href="https://pypi.org/project/pls/">
    <img src="https://img.shields.io/pypi/v/pls" alt="pls on PyPI"/>
  </a>
  <a href="https://www.python.org">
    <img src="https://img.shields.io/pypi/pyversions/pls" alt="Python versions"/>
  </a>
  <a href="https://github.com/dhruvkb/pls/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/dhruvkb/pls" alt="GPL-3.0"/>
  </a>
  <a href="https://pypi.org/project/pls/">
    <img src="https://img.shields.io/static/v1?label=supported%20OS&message=posix,%20win&color=informational" alt="Platforms"/>
  </a>
  <a href="https://dhruvkb.github.io/pls/">
    <img src="https://img.shields.io/static/v1?label=docs&message=dhruvkb/pls:docs&color=informational" alt="Docs"/>
  </a>
  <a href="https://github.com/dhruvkb/pls/actions/workflows/ci.yml">
    <img src="https://github.com/dhruvkb/pls/actions/workflows/ci.yml/badge.svg" alt="CI status"/>
  </a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/dhruvkb/pls/main/readme_assets/demo.png" alt="Demo of `pls`"/>
</p>

`pls` is a prettier `ls` for the pros.

The "p" stands for
- pretty (the output from `pls` surely looks better)
- programmer (`pls` is geared towards developers)
- professional (`pls` can be extensively tweaked by the pros)
- Python (`pls` is written in Python!)

Just pick whichever helps you remember the command name.

It works in a manner similar to `ls`, in  that it lists directories and files in
a given directory, but it adds many more
[developer-friendly features](https://dhruvkb.github.io/pls/features).

> ⚠️ Note that `pls` is not a replacement for `ls`. `ls` is a tried, tested and
trusted command with lots of features. `pls`, on the other hand, is a simple
tool for people who just want to see the contents of their directories.

## Documentation

We have some very beautiful [documentation](https://dhruvkb.github.io/pls) over
on our GitHub pages site. These docs are built from the
[`docs` branch](https://github.com/dhruvkb/pls/tree/docs) in the same
repository, so contributions to the docs are most welcome.

The docs contain information on almost everything, including but not limited to
the following:

- [installation, updates and usage](https://dhruvkb.github.io/pls/get_started)
- [features and CLI options](https://dhruvkb.github.io/pls/features)
- [reference](https://dhruvkb.github.io/pls/reference)
- [contribution](https://dhruvkb.github.io/pls/contribution)

---

🚧 Everything below this line will eventually be transferred to the
[documentation](https://dhruvkb.github.io/pls).

## Features

`pls` provides many features over  `ls` command. `pls` can:

- show Nerd Font icons or emoji next to files and directories making it easier to read the output
- colour output to elevate important files or dim unimportant ones
- use a more nuanced approach to hidden files than plainly hiding files with a leading dot `.`
- group directories and shows them all before files
- ignore leading dots `.` and normalise case when sorting files
- align files names by first character
- show technical two-letter Git status for files and directories
- cascade formatting rule specs by based on specificity levels
- read [`.pls.yml`](.pls.yml) files from the directory to augment its configuration
- show more details like permissions, owner and size in columns
- link files and hide derived files behind the main ones

The icon, color and most behaviour in the application can be [configured using
plain-text YAML files](src/pls/data/README.md) for the pros who prefer to tweak
their tools.
