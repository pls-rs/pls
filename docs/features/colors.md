---
lang: en-GB
title: Colors
description: >-
  pls renders directories and files in a number of different colors, making the
  most of your 256-color terminal.
---

# Colors

Colors are integral to developers. When your primary mode of interaction with
computers is via text, colors make a lot of difference. Modern terminals can
display as many as 256-colors and `pls` can make use of every one of them!

In most cases, `pls` uses the standard color of the language for source code
files.

## Configurations

`pls` comes with
[built-in configuration](https://github.com/dhruvkb/pls/blob/main/src/pls/data/node_specs.yml)
for a number of languages, tools and file types.

- **Python:**  
  Package managers: Poetry and Pipenv  
  Code quality tools: Flake8 and pre-commit

- **Ruby:**  
  Package managers: Bundler  
  Code quality tools: Rubocop

- **JavaScript:**  
  Package managers: Yarn, npm and pnpm  
  Code quality tools: ESLint  
  Frameworks: Vue.js

- TypeScript
- Docker / Docker Compose
- Make / Rake / Just

This enables a highly colorful output where a files colors correspond to the
name and type of the file or directory (directories are cyan by default).

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

Unimportant files (like `poetry.lock` here) are dimmed.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">data</span><span style="color: #156667; text-decoration-color: #156667">/</span>          
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">enums</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">fs</span><span style="color: #156667; text-decoration-color: #156667">/</span>            
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">models</span><span style="color: #156667; text-decoration-color: #156667">/</span>        
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">__init__.py</span>    
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">args.py</span>        
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">exceptions.py</span>  
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">main.py</span>        
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">state.py</span>       
<span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold"></span>   <span style="color: #3776ab; text-decoration-color: #3776ab; font-weight: bold">table.py</span>       
</code></pre>
</div>

Python file are colored in blue <ColorPreview color="#3776ab"/>.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">api</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">dist</span><span style="color: #156667; text-decoration-color: #156667">/</span>                  
<span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">node_modules/</span>          
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">public</span><span style="color: #156667; text-decoration-color: #156667">/</span>                
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">src</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>   <span style="color: #f7df1e; text-decoration-color: #f7df1e">babel.config.js</span>        
   capacitor.config.json  
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>  <span style="color: #7b852a; text-decoration-color: #7b852a">.</span><span style="color: #f7df1e; text-decoration-color: #f7df1e">eslintrc.js</span>            
  <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
   ionic.config.json      
<span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">package-lock.json</span>      
   package.json           
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>   <span style="color: #f7df1e; text-decoration-color: #f7df1e">postcss.config.js</span>      
   README.md              
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>   <span style="color: #f7df1e; text-decoration-color: #f7df1e">tailwind.config.js</span>     
   tsconfig.json          
<span style="color: #f7df1e; text-decoration-color: #f7df1e"></span>   <span style="color: #f7df1e; text-decoration-color: #f7df1e">vue.config.js</span>          
</code></pre>
</div>

JavaScript files are yellow <ColorPreview color="#f7df1e"/>.

<div 
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">components</span><span style="color: #156667; text-decoration-color: #156667">/</span>               
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">compositions</span><span style="color: #156667; text-decoration-color: #156667">/</span>             
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">models</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">pages</span><span style="color: #156667; text-decoration-color: #156667">/</span>                    
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">router</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">store</span><span style="color: #156667; text-decoration-color: #156667">/</span>                    
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">style</span><span style="color: #156667; text-decoration-color: #156667">/</span>                    
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">support</span><span style="color: #156667; text-decoration-color: #156667">/</span>                  
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tokens</span><span style="color: #156667; text-decoration-color: #156667">/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">views</span><span style="color: #156667; text-decoration-color: #156667">/</span>                    
<span style="color: #4fc08d; text-decoration-color: #4fc08d">﵂</span>   <span style="color: #4fc08d; text-decoration-color: #4fc08d">App.vue</span>                   
<span style="color: #2496ed; text-decoration-color: #2496ed"></span>   <span style="color: #2496ed; text-decoration-color: #2496ed">Dockerfile</span>                
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">main.ts</span>                   
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">registerServiceWorker.ts</span>  
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">shims-png.d.ts</span>            
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">shims-svg.d.ts</span>            
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">shims-vue.d.ts</span>            
<span style="color: #3178c6; text-decoration-color: #3178c6"></span>   <span style="color: #3178c6; text-decoration-color: #3178c6">shims-vuex.d.ts</span>           
</code></pre>
</div>

TypeScript and Docker files are also blue (<ColorPreview color="#3178c6"/>
and <ColorPreview color="#2496ed"/> respectively) but these blues are different
from each other, and also from Python's blue <ColorPreview color="#3776ab"/>.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">colorls</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">yaml</span><span style="color: #156667; text-decoration-color: #156667">/</span>            
<span style="color: #cc342d; text-decoration-color: #cc342d"></span>   <span style="color: #cc342d; text-decoration-color: #cc342d">colorls.rb</span>       
ﲵ   tab_complete.sh  
</code></pre>
</div>

Ruby files are red <ColorPreview color="#cc342d"/>.

## Customisation

To add more color schemes, you can do either of the following.

- Extend the configuration locally using a local `.pls.yml` file.  
  For the majority of use cases, this is the preferred option. Using `pls`'s
  powerful extensibility, you can define color schemes for your projects
  (among other things).

- Make a pull-request to `pls`.  
  This only applies if the language is fairly popular and the inclusion of the
  config adds value for a lot of developers. The `pls` OOBE is intentionally
  minimal.

Colors are set via node specs. Refer to the
[docs for defining node specs](../reference/node_specs).
