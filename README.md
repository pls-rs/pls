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
    <img src="https://img.shields.io/static/v1?label=supported%20OS&message=mac,%20win&color=informational" alt="Supported OS"/>
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

It works in a manner similar to `ls`, in  that it lists directories and files in a given directory, but it adds many more [developer-friendly features](#features).

Note that `pls` is not a replacement for `ls`. `ls` is a tried, tested and trusted command with lots of features. `pls`, on the other hand, is a simple tool for people who just want to see the contents of their directories.

## Documentation

We have some very beautiful [documentation](https://dhruvkb.github.io/pls) over
on our GitHub pages site. These docs are built from the
[`docs` branch](https://github.com/dhruvkb/pls/tree/docs) in the same
repository, so contributions to the docs are most welcome.

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

The icon, color and most behaviour in the application can be [configured using plain-text YAML files](src/pls/data/README.md) for the pros who prefer to tweak their tools.

## Upcoming features

In the future `pls` will be able to

- generate visibility rules by parsing `.gitignore`
- add MIME type as another method for matching files to specs
- use complete path based matching for files
- generate tree-like output for subdirectories

If you want to help implement any of these features, feel free to submit a PR. `pls` is free and open-source software.

## Comparison with similar tools

There are a lot of `ls` replacements. Here are some of the most popular ones.

- [`exa`](https://github.com/ogham/exa)
- [`lsd`](https://github.com/Peltoche/lsd)
- [`colorls`](https://github.com/athityakumar/colorls)
- [`ls-go`](https://github.com/acarl005/ls-go)

`pls` aims to stand out because of some very specific choices.

- Does not intend to replace `ls`. `pls`, as a command, is just as easy to type.
- Targets a more tech-savvy audience in its [features](#features).
- Intelligently [maps file type](src/pls/data/README.md). Just comparing the file extension would be too generic.
- Meticulously chosen iconography for the appreciating eyes.
- Highly customisable at a project level using a simple [`.pls.yml`](.pls.yml) file.
- Built in a friendly language, Python. This makes it easy to fork and change it yourself.

## Installation

To get the best of `pls`, [install a Nerd Font](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md#font-installation) on your computer. [Nerd Fonts](https://www.nerdfonts.com) come patched with many icons from different popular icon sets. If you're a "pro" (the target audience for `pls`) these fonts are basically a must.

`pls` is a pure-Python codebase and is deployed to PyPI. So installing it on any system with a supported Python version is quite straightforward.

```shell
$ python3 -m pip install --user pls
```

There are no native packages _yet_.

## Usage

`pls` has a very simple API with easy to memorise flags. There are no mandatory arguments. Just run `pls` anywhere on your disk.

```shell
$ pls
```

There are a few optional arguments and flags you can use to tweak the behaviour. You can see the complete list of arguments and their description by passing the `--help` or `-h` flags.

```shell
$ pls --help
```

### Directory

The only positional argument is a directory. Pass this to see the contents of a different folder rather than the current working directory.

```shell
$ pls path/to/somewhere/else
```

### Icons

`pls` supports many icons for popular languages out of the box and will show icons by default. If you don't have a Nerd Font (why?), you can switch to emoji icons using `--icons emoji` or `-iemoji`. Be warned they are quite bad. If you are a sad person, you turn icons off using `--icon none` or `-inone`.

**Note:** The built-in icon configuration is intentionally lean. The whole idea is for `pls` to be [customisable by you](src/pls/data/README.md).

### Filtering

You can choose to hide files or folders from the output using `--no-files` and `--no-dirs` respectively. Passing both will lead to a blank output. On the other hand if you want to see files and directories that `pls` would not show otherwise, pass `--all`.

### Sorting

By default `pls` will place all directories first, followed by files with both sorted alphabetically from A to Z. You can prevent folders from being first by passing the `--no-dirs-first` flag. You can change the sort to go from Z to A using `--sort desc` or `-sdesc`. Leading dots are ignored during sorting.

### Alignment

A lot of code related files start with a leading dot `.` for no valid reason. `pls` by default

- moves their name left by one character to line up the actual alphabets
- dims their leading dot

If you don't like this, you can set `--no-align` to turn off all this behaviour in one swoop.

### Details

When you need more infomation about your files, pass the `--details` flag. This expands the list into a table, with

- permissions
- owner name
- size
- Git status _(if available)_

added to the output. The permissions are presented as `rwx` triplets. The size is presented in binary compound-units (the ones with the "i" like "*iB"). You can switch to decimal units by passing `--units decimal` or `-udecimal`. This flag has no effect unless the `--detail` flag is passed too.
