<h1 align="center">
  <img height="128px" src="https://raw.githubusercontent.com/dhruvkb/pls/main/readme_assets/pls.svg"/>
</h1>

<p align="center">
  <a href="https://pypi.org/project/pls/">
    <img src="https://img.shields.io/pypi/v/pls" alt="pls on PyPI"/>
  </a>
  <a href="https://www.python.org">
    <img src="https://img.shields.io/pypi/pyversions/pls" alt="Python ^3.9"/>
  </a>
  <a href="https://github.com/dhruvkb/pls/blob/main/LICENSE">
    <img src="https://img.shields.io/pypi/l/pls" alt="GPL-3.0-or-later"/>
  </a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/dhruvkb/pls/main/readme_assets/demo.png" alt="Demo of `pls`"/>
</p>

`pls` is a better `ls` for developers. The "p" stands for ("pro" as in "professional"/"programmer") or "prettier".

It works in a manner similar to `ls`, in  that it lists directories and files in a given directory, but it adds many more developer-friendly features.

Note that `pls` is not a replacement for `ls`. `ls` is a tried, tested and trusted tool with lots of features. `pls`, on the other hand, is a simple tool for people who just want to see the contents of their directories.

## Features

`pls` provides many features over  `ls` command. `pls` can:

- show Nerd Font icons or emoji next to files and directories making it easier to grep the output
- colour output to further distinguish important files
- use a more nuanced approach to hidden files than plainly hiding files with a leading dot `.`
- groups directories and shows them all before files
- ignores leading dots `.` and normalises case when sorting files
- cascade specs by based on specificity levels
- read `.pls.yml` files from the directory to augment its configuration
- show more details like permissions, owner and size in columns

The icon, color and most behaviour in the application can be configured using plain-text YAML files for the pros who prefer to tweak their tools.

## Upcoming features

In the future `pls` will be able to

- generate visibility rules by parsing `.gitingore`
- add MIME type as another method for matching files to specs
- use complete path based matching for files
- link files and hide derived files behind the main ones
- support for tree-like output

If you want to help implement any of these features, feel free to submit a PR. `pls` is free and open-source software.
