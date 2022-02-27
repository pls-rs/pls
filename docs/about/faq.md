---
lang: en-GB
title: FAQ
description: >-
  Read some frequently asked questions about pls and the answers to these 
  questions to know more about the tool.
---

# FAQ

Here are some questions about `pls` that I assume would come up pretty
frequently and the answers to those questions.

## Is this a replacement for `ls`?

**No.** `pls` is a separate tool for prettier output intended for human
consumption.

- `pls` has a very different API compared to `ls`. It has fewer but easy to
  memorize options.
- `pls` focuses heavily on the making the output human-readable, making it a bad
  fit for scripting.

We recommend keeping `ls` around. `ls` is a tried, tested and trusted tool with
lots of features.

## Why does this even exist then?

If your work involves writing code, you are most likely working in IDEs or in
the terminal. IDEs do all sorts of optimisations to help you see your files
clearly and find them faster. `ls` does none of these.

`pls` aims to be the closest thing to an IDE-like file panel inside a terminal.

## Does `pls` support Windows?

**Yes**, with caveats.

`pls` runs on Windows with a reduced feature set but that is largely a result of
Windows being an inferior operating-system. To experience the true power of CLI,
we recommend using a POSIX-compatible OS like Linux or macOS.

## Why Python and not _&lt;lang&gt;_?

<!-- Every once in a while, a developer will come up to me and ask, "Dhruv, why
is `pls` written in Python?" They say it just like that. "Why is `pls` written
in Python?"

"Why Python? Why not Rust? Why not Golang? Why not <my favourite language>? I
mean, surely, there are languages faster and more up to the task of making CLI
apps. Well, I'll tell you why I used Python: it's an awesome language, OK? And I
am a Python genius!" -->

**Because Python is awesome.** No seriously, here are some arguments in favour
of Python.

- `pls` runs directly from source making it easy to modify and debug locally.
- `pls` be installed universally without having to ship binaries for different
  platforms.
- The codebase is easy to navigate, understand and contribute to by beginners
  and experts alike.
- High-level languages like Python facilitate rapid prototyping of new features.
  The REPL is powerful.
- Python has a huge, mature library of packages in PyPI. Notably, `pls` uses
  [Rich](https://github.com/Textualize/rich).

If `pls` becomes the `ls`-replacement with the most GitHub stars, I'll consider
rewriting it in Rust 😉.
