---
lang: en-GB
title: Icons
description: >-
  pls can show Nerd Font icons or emoji next to files and directories to make
  them easier to distinguish.
---

# Icons

`pls` can show Nerd Font icons or emoji next to files and directories making it
easier to read the output. In effect, `pls` enables you to annotate your files
and directories with the full set of 3693 Nerd Font icons and 3633 emojis!

The icons are meticulously chosen and intelligently mapped to nodes using specs
which consider many aspects of the node rather than just the extension.

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

Notice how directories and files have these very pretty icons to their left?
These are Nerd Font icons.

## Options

`pls` shows Nerd Font icons by default. But this can be changed to emoji or even
turned off by using the `--icons`/`-i` flag.

- `nerd`: show Nerd Font icons (default)

- `emoji`: show emojis

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #156667; text-decoration-color: #156667">🎁</span>  <span style="color: #156667; text-decoration-color: #156667">dist/</span>                   
<span style="color: #2aa198; text-decoration-color: #2aa198">📁</span>  <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>          
<span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">💻</span>  <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                    
<span style="color: #2aa198; text-decoration-color: #2aa198">🧪</span>  <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                  
🧹 <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                  
⏪ <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore               
🏃  justfile                
⚖️   LICENSE                 
<span style="font-style: italic">⚪️</span> <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                 
<span style="color: #415f66; text-decoration-color: #415f66">🔒</span>  <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>             
🧹 <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml  
📦  pyproject.toml          
📄  README.md               
</code></pre>
</div>

- `none`: turn off icons

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"> <span style="color: #156667; text-decoration-color: #156667">dist/</span>                   
 <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>          
 <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                    
 <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                  
<span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                  
<span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore               
 justfile                
 LICENSE                 
<span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                 
 <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>             
<span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml  
 pyproject.toml          
 README.md               
</code></pre>
</div>

## Configuration

Out of the box, `pls` includes
[Nerd Font icons](https://github.com/dhruvkb/pls/blob/main/src/pls/data/nerd_icons.yml)
and
[emojis](https://github.com/dhruvkb/pls/blob/main/src/pls/data/emoji_icons.yml)
for a large number of file types.

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

Here you can see how files and directories map precisely to very topical icons.
It should be noted how icons map to file names as well and are not just based on
the extension. Notice the icons for `.gitignore`, `justfile` for the task
runner [Just](https://just.systems) and `LICENSE`.

Files for code quality tools like `.pre-commit-config.yaml` and `.flake8` have
the broom. Package files have a box and the lock files have a lock.

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

Python source code files have the Python logo on them.

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

JavaScript files say JS. Notice how `package.json` and `capacitor.config.json`
have the same extension but different icons.

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

Docker files have the whale. TypeScript files say TS. Vue.js SFC files have the
logo of the Vue.js framework.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">colorls</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">yaml</span><span style="color: #156667; text-decoration-color: #156667">/</span>            
<span style="color: #cc342d; text-decoration-color: #cc342d"></span>   <span style="color: #cc342d; text-decoration-color: #cc342d">colorls.rb</span>       
ﲵ   tab_complete.sh  
</code></pre>
</div>

Ruby files have the red ruby. Shell files have the icon for a CLI shell.

## Customisation

To add more icons, you can do either of the following.

- Extend the configuration locally using a local `.pls.yml` file.  
  For the majority of use cases, this is the preferred option. Using `pls`'s
  powerful extensibility, you can define new icons and override existing ones
  for your projects (among other things).

  In the examples above, the icons for `src/` and `tests/` were defined by the
  local config file.

- Make a pull-request to `pls`.  
  This only applies if the file type is fairly common and the inclusion of the
  config adds value for a lot of developers. The `pls` OOBE is intentionally
  minimal.

Refer to the [docs for configuring icons](../reference/icons).

## Requirements

- For Nerd icons, a compatible [Nerd Font](https://www.nerdfonts.com/) will need
  to be installed. Instructions to do so can be found in the
  [project's `readme.md` file](https://github.com/ryanoasis/nerd-fonts).
- Emoji icons will only work if your computer and terminal are equipped to
  render them.
