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
| cat   | Directory/file |
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

## Preferences

**CLI flags:** `--sort`/`-s`  
**Config YAML:** `sort`

This is a [list of enum field](../reference/prefs.md#lists). This means you can
pass the CLI flag multiple times, with a different value after the flag, and
they will all be collected, in sequence. These are the valid values.

- `cat`, `name`: sort by the canonical name of the node, with directories first
  (default)

  ```shellsession
  $ pls # default
  $ pls -s cat -s name
  ```

  ```yml
  prefs:
    sort:
      - cat
      - name
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
   CODE_OF_CONDUCT.md     
   CONTRIBUTING.md        
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
ﰌ   justfile               
   LICENSE                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
   <span style="text-decoration: underline">README.md</span>              
</code></pre>
</div>

- individual keys: add series of tie-breakers; Using multiple sorting fields
  allows you to sort using successive fields if the previous keys are all equal.

  ```shellsession
  $ pls -s ext -s name
  ```

  ```yml
  prefs:
    sort:
      - ext
      - name
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
ﰌ   justfile               
   LICENSE                
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
   CODE_OF_CONDUCT.md     
   CONTRIBUTING.md        
   <span style="text-decoration: underline">README.md</span>              
   pyproject.toml         
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
</code></pre>
</div>

- individual keys with `-` suffix: reverse the sort for that particular key.

  ```shellsession
  $ pls -s ext- -s name
  ```

  ```yml
  prefs:
    sort:
      - ext-
      - name
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;">  <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
   CODE_OF_CONDUCT.md     
   CONTRIBUTING.md        
   <span style="text-decoration: underline">README.md</span>              
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
<span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
ﰌ   justfile               
   LICENSE                
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
</code></pre>
</div>

## Reference

Describing what these details mean is beyond the scope of this guide. How `pls`
is concerned with them is described below.

### Name (`name`)

The name of the file is used for sorting after normalisation. This involves the
following steps:

- removing all leading dots from the file name
- converting the name to lowercase
