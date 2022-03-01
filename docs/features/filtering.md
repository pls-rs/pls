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

## Options

There are a number of flags that control the filter criteria.

- Pass the `--all`/`-a` flag to change the importance thresholds. See the
  [docs on importance](./importance) for more information on this option.

- Pass the `--no-dirs` flag to hide all directories from the output, and show
  files.

```
$ pls --no-dirs
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

- Pass the `--no-files` flag to hide all files from the output, and only show
  directories.

```
$ pls --no-files
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span> 
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>           
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
</code></pre>
</div>

- Pass the `--exclude`/`-e` option with a regular expression to hide all files
  matching the pattern.

```
$ pls -e '.*\.ya?ml'
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

- Pass the `--only`/`-o` option with a regular expression to only show files
  matching the pattern.

```
$ pls -o '.*\.ya?ml'
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
</code></pre>
</div>

## Reference

### Importance (`--all`)

See the [docs for importance](./importance) for more information on this option.

### Regex (`--exclude`/`-e` and `--only`/`-o`)

Both the 'exclude' and 'only' flags take regular expressions that are matched
against the node name. These expressions match the node name from the start so
for matches targeting substrings not in the beginning should be prefixed with
a wildcard match at the start `.*`.

::: tip
Wrap the regular expression in single quotes to prevent the shell from tampering
with it.
:::

Passing both `--only`/`-o` and `--exclude`/`-e` will lead to a combined effect
where only files matching both conditions will be shown.

```
$ poetry run pls -e 'README' -o '.*\.md'
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;">   CODE_OF_CONDUCT.md 
   CONTRIBUTING.md    
</code></pre>
</div>
