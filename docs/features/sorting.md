---
lang: en-GB
title: Sorting
description: >-
  pls can sort files and directories based on a number of attributes such as 
  name, extension, timestamps etc.
---

# Sorting

When listing lots of files and directories, sorting can help find the right file
faster. `pls` can sort files based on the following attributes.

| Key   | Meaning        |
| ----- | -------------- |
| name  | Name           |
| ext   | File extension |
| inode | inode          |
| links | Link#          |
| type  | Type character |
| size  | Size           |
| ctime | Created at     |
| mtime | Modified at    |
| atime | Accessed at    |

By default, nodes are sorted by their name (ignoring case and excluding leading
dots `.`). A different criterion can be specified using the `--sort`/`-s` flag.

## Options

The `--sort`/`-s` flag takes any of options.

- Pass any single key with the `--sort`/`-s` flag to sort the output based on
  the value of the field. This will use the order A &rarr; Z for strings and
  0 &rarr; 9 for numbers.

```:no-line-numbers
$ pls -s ext
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
ﰌ   justfile               
   LICENSE                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
   README.md              
   pyproject.toml         
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
</code></pre>
</div>

- To reverse the sort order append a hyphen `-` after the key name. This will
  reverse the sorting order to be Z &rarr; A for strings and 9 &rarr; 0 for
  numbers.

```:no-line-numbers
$ pls -s ext-
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
   README.md              
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
   LICENSE                
ﰌ   justfile               
</code></pre>
</div>

By default, `pls` also puts directories above files and sorts them separately.
To mix files and directories, pass the `--no-dirs-first` option.

```:no-line-numbers
$ pls --no-dirs-first
```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
ﰌ   justfile               
   LICENSE                
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
   README.md              
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
</code></pre>
</div>

## Reference

Describing what these details mean is beyond the scope of this guide. How `pls`
is concerned with them is described below.

## Name (`name`)

The name of the file is used for sorting after normalisation. This involves the
following steps:

- removing all leading dots from the file name
- converting the name to lowercase

Based on this normalisation, `.pls.yml` is placed after `LICENSE`.

::: tip
Regardless of sorting choices, the name field acts as the tie-breaker.
:::
