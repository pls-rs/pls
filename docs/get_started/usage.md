---
lang: en-GB
title: Usage
description: >-
  Using pls is extremely easy, thanks to the easy-to-use command-line API. It
  even comes with help built-in.
---

# Usage

## Invocation

To run `pls`, type the command into any terminal.

```shellsession
$ pls
```

By default, `pls` lists the contents of the current working directory, but you
can pass a different file or directory to `pls` as an argument.

If a directory path is passed, all the files and directories within that
directory are listed. This is useful to see what's in the folder.

```shellsession
$ pls <directory>
```

If a file path is passed, only the file itself is listed. This is useful in
conjunction with the `--details` flag to see more attributes about the file.

```shellsession
$ pls <file>
```

## Help

To get help, run `pls` with the `--help`/`-h` flag or _read this documentation_!

```shellsession
$ pls --help
$ pls -h
```
