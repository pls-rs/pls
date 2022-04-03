---
lang: en-GB
title: Multi-cols
description: >-
  When not displaying extended information about nodes, pls allows you to place
  more files on the screen in a condensed multi-column layout.
---

# Multi-cols

Screen space is a prized commodity. When displaying only file names, without the
extra information, it might help to print more files on the screen placed in a
2D grid instead of a vertical 1D list, like how `ls` does it. `pls` allows you
to print the file names in multiple columns, as wide as your terminal app or
monitor goes.

## Preferences

**CLI flags:** `--multi-cols`/`--no-multi-cols`
**Config YAML:** `multi_cols`

This is a [boolean field](../reference/prefs.md#booleans).

- `--no-multi-cols`/`false`: render files in one column (default)

  ```shellsession
  $ pls # default
  $ pls --no-multi-cols
  ```

  ```yml
  prefs:
    multi_cols: false
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>          
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                    
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                  
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                  
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore               
ﰌ   justfile                
   LICENSE                 
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                 
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>             
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml  
   pyproject.toml          
   README.md               
</code></pre>
</div>

- `true`

  ```shellsession
  $ pls --multi-cols
  ```

  ```yml
  prefs:
    multi_cols: true
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>  <span style="color: #156667; text-decoration-color: #156667">dist/</span>                                 ﰌ  justfile                            
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>  <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>                          LICENSE                             
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>  <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                                  <span style="font-style: italic"></span> <span style="color: #415f66; text-decoration-color: #415f66">.</span><span style="font-style: italic">pls.yml</span>                             
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>  <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                                <span style="color: #415f66; text-decoration-color: #415f66"></span>  <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>                         
  CODE_OF_CONDUCT.md                     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml              
  CONTRIBUTING.md                         pyproject.toml                      
 <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                                  README.md                           
 <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore                                                                    
</code></pre>
</div>

::: tip
While the output above shows only two columns, the number of columns is
effectively determined by the width of your terminal. As you can see, this
feature becomes very useful when you have a wider screen and want to make the
most of it.
:::

:::warning
Multi-cols takes precedence over collapse, meaning that the collapse
functionality is suspended when rendering nodes in multiple columns. The
`--collapse`/`-c` flag has no effect.
:::
