---
lang: en-GB
title: Configuration
description: >-
  Extensibility is at the core of pls. Being a tool for pros, pls allows for
  very deep customisation at a very deep level.
---

# Configuration

Since its inception, `pls` has been a tool designed for pros. This goal
manifests most evidently in the ability to highly customise `pls` for your very
specific workflow.

## Tiers

`pls` can be configured at three levels, in a cascading manner.

- built-in
- user-level
- project-level

Even within a level, a node can match multiple different specs, rules from which
are then sequentially applied to the node in the order in which they are
defined. This enables higher-specificity specs to override attributes from the
lower-specificity ones.

### Built-in

`pls` ships with a very lean configuration out-of-the-box. This configuration
makes it directly usable for the majority of users without bogging down the
performance in a lot of unnecessary computation.

You can see the built-in configuration
[here](https://github.com/dhruvkb/pls/tree/main/src/pls/data/).

### User-level

`pls` can be configured at the user level by placing a `.pls.yml` file in the
user's home directory, denoted by `~` on POSIX and `C:\Users\*` on Windows.

The configuration described by this file extends and overrides the built-in
configuration.

You can see my personal configuration
[here](https://github.com/dhruvkb/dotfiles/blob/main/pls/.pls.yml). It's
symlinked into `~/.pls.yml`.

### Project-level

`pls` can also be configured per-project by placing a `.pls.yml` file in the
project. `pls` will look for this config file in the working directory, 4 levels
above the working directory (configurable with the `--depth` parameter) and in
the Git root (if the project is Git-tracked).

The configuration described by this file extends and overrides the user-level
and built-in configurations.

You can take a look at the project configuration of `pls` itself
[here](https://github.com/dhruvkb/pls/blob/main/.pls.yml).

## `.pls.yml`

The `.pls.yml` configuration file consists of five parts.

- `emoji_icons`
- `nerd_icons`
- `node_specs`
- `prefs`
- `constants`

### Node specs

`node_specs` is a list of node specifications that describe the methods for
identifying nodes and their rendering configuration.

Read more [in the `node_specs` docs](./node_specs).

### Icons (emoji and Nerd Font)

Both `emoji_icons` and `nerd_icons` are dictionaries mapping icon names to icon
glyphs.

Read more [in the `*_icons` docs](./icons).

## Schema validation

Writing YAML manually can be error-prone. To make the process simpler, we
provide a YAML-format JSON-schema of the file for validation, hosted at
[this URL](https://raw.githubusercontent.com/dhruvkb/pls/main/src/pls/data/schema/pls_config.yml).

```
https://raw.githubusercontent.com/dhruvkb/pls/main/src/pls/data/schema/pls_config.yml
```

Refer to your IDE/editor docs for instructions on how to associate YAML files
with a schema. This will help you write mostly valid config files.

- [IntelliJ IDEA](https://www.jetbrains.com/help/idea/json.html#ws_json_schema_add_custom)
- [Visual Studio Code +\_YAML Language Support by Red Hat](https://github.com/redhat-developer/vscode-yaml#associating-schemas)

You can also compare your configuration files against the schema using an online
validator such as [Hyperjump](https://json-schema.hyperjump.io).
