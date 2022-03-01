---
lang: en-GB
title: Collapse
description: >-
  When the code contains auto-generated files such as lockfiles for dependencies
  and compiled files from higher-order languages, pls can nest them for you.
---

# Collapse

Not all files are created equally. Some are written by people with great care
and effort while others are generated automatically.

`pls` can nest generated files under the handwritten ones when using the
`--collase`/`-c` flag.

## Options

`pls` does not collapse files by default. You can choose to collapse files by
passing the `--collapse`/`-c` flag.

```
$ pls --collapse
$ pls -c
```

You can hide collapsed files altogether by passing the `--collapse`/`-c` flag
twice.

```
$ pls --collapse --collapse
$ pls -c -c
$ pls -cc
```

### Configuration

Out of the box, `pls` can collapse the following files:

- `pyproject.toml`
  - `poetry.lock`
- `Pipfile`
  - `Pipfile.lock`
- `Gemfile`
  - `Gemfile.lock`
- `package.json`
  - `package-lock.json`
  - `yarn.lock`
  - `pnpm-lock.yaml`
- file with extension `ts`
  - file with same name and extension `js`
- file with extension `tsx`
  - file with same name and extension `jsx`
- file with extension `scss`, `sass` or `less`
  - file with same name and extension `css`

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667"></span>  <span style="color: #156667; text-decoration-color: #156667"> dist/</span>                  
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>  <span style="color: #2aa198; text-decoration-color: #2aa198"> readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>  <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"> src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>  <span style="color: #2aa198; text-decoration-color: #2aa198"> tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
ﰌ   justfile               
   LICENSE                
<span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
   pyproject.toml         
<span style="color: #415f66; text-decoration-color: #415f66"></span>  <span style="color: #415f66; text-decoration-color: #415f66"> └─ poetry.lock</span>         
   README.md              
</code></pre>
</div>

Notice how collapsing puts the `poetry.lock` file below `pyproject.toml`.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #e34f26; text-decoration-color: #e34f26"></span>  <span style="color: #e34f26; text-decoration-color: #e34f26"> index.html</span>           
   package.json         
<span style="color: #415f66; text-decoration-color: #415f66"></span>  <span style="color: #415f66; text-decoration-color: #415f66"> └─ package-lock.json</span> 
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>  <span style="color: #3178c6; text-decoration-color: #3178c6"> scripts.ts</span>           
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>  <span style="color: #7b852a; text-decoration-color: #7b852a"> └─</span><span style="color: #f7df1e; text-decoration-color: #f7df1e"> scripts.js</span>        
<span style="color: #cc6699; text-decoration-color: #cc6699"></span>  <span style="color: #cc6699; text-decoration-color: #cc6699"> styles.scss</span>          
<span style="color: #1572b6; text-decoration-color: #1572b6"></span>  <span style="color: #0a4e76; text-decoration-color: #0a4e76"> └─</span><span style="color: #1572b6; text-decoration-color: #1572b6"> styles.css</span>        
</code></pre>
</div>

Notice how awesome collapse looks when there's a lot of content to work with.

### Customisation

To define more collapse rules, you can do either of the following.

- Extend the configuration locally using a local `.pls.yml` file.  
  For the majority of use cases, this is the preferred option. Using `pls`'s
  powerful extensibility, you can define collapse rules for your projects
  (among other things).

- Make a pull-request to `pls`.  
  This only applies if the language is fairly popular and the inclusion of the
  config adds value for a lot of developers. The `pls` OOBE is intentionally
  minimal.
