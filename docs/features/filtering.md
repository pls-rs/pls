---
lang: en-GB
title: Filtering
description: >-
  pls can filter the output using regular expressions to precisely control the
  files and folders you want to see.
---

# Filtering

When looking for files, filtering is one of the powerful tools in your arsenal.
`pls` can filter files using a number of techniques, including the most powerful
of them all, regular expressions.

## By type

`pls` allows you to selectively filter out directories or files from the output.

### Files

**CLI flags:** `--files`/`--no-files`  
**Config YAML:** `files`

This is a [boolean field](../reference/prefs.md#booleans).

- `--files`/`true`: show files in the output (default)

  ```shellsession
  $ pls # default
  $ pls --files
  ```

  ```yml
  prefs:
    files: true
  ```

- `--no-files`

  ```shellsession
  $ pls --no-files
  ```

  ```yml
  prefs:
    files: false
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span> 
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>           
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
</code></pre>
</div>

### Directories

**CLI flags:** `--dirs`/`--no-dirs`  
**Config YAML:** `dirs`

This is a [boolean field](../reference/prefs.md#booleans).

- `--dirs`/`true`: show directories in the output (default)

  ```shellsession
  $ pls # default
  $ pls --dirs
  ```

  ```yml
  prefs:
    files: true
  ```

- `--no-dirs`

  ```shellsession
  $ pls --no-dirs
  ```

  ```yml
  prefs:
    dirs: false
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;">   CODE_OF_CONDUCT.md     
   CONTRIBUTING.md        
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
ﰌ   justfile               
   LICENSE                
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
   README.md              
</code></pre>
</div>

## By name

There are a number of flags that control the filter criteria.

### Exclude

**CLI flags:** `--exclude`/`-e`  
**Config YAML:** `exclude`

This is a [string field](../reference/prefs.md#strings).

Pass the `--exclude`/`-e` option with a regular expression to hide all files
matching the pattern.

```shellsession
$ pls -e '.*\.ya?ml'
```

```yml
prefs:
  exclude: .*\.ya?ml
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>     
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>               
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>             
   CODE_OF_CONDUCT.md 
   CONTRIBUTING.md    
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8             
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore          
ﰌ   justfile           
   LICENSE            
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>        
   pyproject.toml     
   README.md          
</code></pre>
</div>

### Include

**CLI flags:** `--only`/`-o`  
**Config YAML:** `only`

This is a [string field](../reference/prefs.md#strings).

Pass the `--only`/`-o` option with a regular expression to only show files
matching the pattern.

```shellsession
$ pls -o '.*\.ya?ml'
```

```yml
prefs:
  only: .*\.ya?ml
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
</code></pre>
</div>

### Reference

Both the `exclude` and `only` options take regular expressions that are matched
against the node name. The match is performed from the start of the node name so
for matches targeting substrings not in the beginning should be prefixed with
a wildcard match at the start `.*`.

::: warning
Filters are combined by AND operations. Thus, setting both `only` and `exclude`
(either via CLI or YAML config) will lead to a combined effect where only files
and directories satisfying both conditions will be shown.
:::

::: tip
In the CLI, wrap the regular expression in single quotes to prevent the shell
from tampering with it. In YAML, skip the quotes altogether or use single quotes
to prevent the escape codes from being parsed.
:::

```shellsession
$ pls -e 'README' -o '.*\.md'
```

```yml
prefs:
  exclude: README
  only: .*\.md
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;">   CODE_OF_CONDUCT.md 
   CONTRIBUTING.md    
</code></pre>
</div>

## By importance

See the [docs for the Importance feature](./importance) for more information on
this filtering option.
