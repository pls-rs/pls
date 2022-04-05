---
lang: en-GB
title: Details
description: >-
  pls can show a lot more information about your files and directories like
  type, perms, user, group, size and Git status.
---

# Details

Sometimes just a list of file names is not enough. When you need more
information, `pls` can deliver. `pls` can query the system for all the following
attributes.

| Standard           | Key   | Meaning            | Note                           |
| ------------------ | ----- | ------------------ | ------------------------------ |
|                    | inode | inode              | Not on Windows                 |
|                    | links | Link#              | Not on Windows                 |
| :white_check_mark: | type  | Type character     |                                |
| :white_check_mark: | perms | Permissions        |                                |
| :white_check_mark: | user  | Owner user         | Not on Windows                 |
| :white_check_mark: | group | Owner group        | Not on Windows                 |
|                    | size  | Size               |                                |
|                    | btime | Created at         | Only on macOS                  |
|                    | ctime | Created/Changed at |                                |
|                    | mtime | Modified at        |                                |
|                    | atime | Accessed at        |                                |
|                    | git   | Git status         | [Conditional](#git-status-git) |

On a decently capable computer (which I'm sure most pros will have), Python is
pretty fast. Even with all details turned on, `pls` runs fast enough that there
isn't a noticeable delay.

## Preferences

**CLI flags:** `--details`/`-d`  
**Config YAML:** `details`

This is a [list of enum field](../reference/prefs.md#lists). This means you can
pass the CLI flag multiple times, with a different value after the flag, and
they will all be collected, in sequence. These are the valid values.

- `none`: show no details (default)

  ```shellsession
  $ pls
  $ pls -d none
  ```

  ```yml
  prefs:
    details: []
  ```

- `std`: show the standard subset of detail fields; For a list of standard keys,
  see the fields with ✅ in the 'Standard' column of the table above.

  ```shellsession
  $ pls -d
  $ pls -d std
  ```

  ```yml
  prefs:
    details:
      - std
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">  Permissions   User    Group       Name                   </span>
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   ﰌ   justfile               
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff      LICENSE                
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff      pyproject.toml         
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff      README.md              
</code></pre>
</div>

- `all`: show all details fields; This is probably too much information, and
  you'll likely need a wider monitor.

  ```shellsession
  $ pls -d all
  ```

  ```yml
  prefs:
    details:
      - all
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">inode    Link#     Permissions   User    Group     Size   Created at           Modified at          Accessed at            Git       Name                   </span>
48331692    22   d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff            <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span>         <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
48332230     4   d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff            <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:41 </span>         <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
48333901     3   d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff            <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:33 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:33 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>         <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
48331711     5   d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff            <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span>         <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
48331679     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   488<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>           <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
48332226     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   245<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:39 </span>           <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
48331682     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   911<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>         ﰌ   justfile               
48331681     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff    34<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>            LICENSE                
48332233     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   237<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>         <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
48332234     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff    33<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:08 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:08 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:43 </span>         <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
48331709     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff     2<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-13 22:10<span style="color: #415f66; text-decoration-color: #415f66">:56 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>           <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
48704366     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   713<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:42 </span>            pyproject.toml         
48704365     1   - <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff     7<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>   <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 23:54<span style="color: #415f66; text-decoration-color: #415f66">:19 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022</span>-02-14 23:46<span style="color: #415f66; text-decoration-color: #415f66">:17 </span>            README.md              
</code></pre>
</div>

- individual keys: selectively enable detail fields; This makes the most sense.
  You can also combine them with `std` include the standard set of columns. For
  a list of keys, refer to the 'Key' column in the table above.

  ```shellsession
  $ pls -d -d mtime
  $ pls -d std -d mtime
  ```

  ```yml
  prefs:
    details:
      - std
      - mtime
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">  Permissions   User    Group   Modified at                Name                   </span>
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span>   <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:33 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
d <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span><span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span> <span style="color: #b58900; text-decoration-color: #b58900">r</span>-<span style="color: #859900; text-decoration-color: #859900">x</span>   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-16 20:10<span style="color: #415f66; text-decoration-color: #415f66">:14 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>   ﰌ   justfile               
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>      LICENSE                
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:08 </span>   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>      pyproject.toml         
- <span style="color: #b58900; text-decoration-color: #b58900">r</span><span style="color: #dc322f; text-decoration-color: #dc322f">w</span>- <span style="color: #b58900; text-decoration-color: #b58900">r</span>-- <span style="color: #b58900; text-decoration-color: #b58900">r</span>--   dhruvkb staff   <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-16 19:55<span style="color: #415f66; text-decoration-color: #415f66">:58 </span>      README.md              
</code></pre>
</div>

:::warning
Details take precedence over multi-cols, meaning that the multi-column layout
is replaced with a table when showing details. The `--multi-cols`/`-m` flag has
no effect.
:::

## Reference

Describing what these details mean is beyond the scope of this guide. How `pls`
is concerned with them is described below.

### inode (`inode`)

Gives the inode number of the file.

::: warning
This column is not available on Windows. Including it will have no effect.
:::

### Link# (`links`)

Gives the number of links pointing to the file. For a directory, this number is
usually quite large as it includes all files within it. `pls` highlights link
count larger than one for files.

::: warning
This column is not available on Windows. Including it will have no effect.
:::

### Type character (`type`)

This is a character that denotes the type of the file.

| Key | Meaning      |
| --- | ------------ |
| `l` | symlink      |
| `d` | directory    |
| `-` | regular file |
| `p` | FIFO         |
| `s` | socket       |
| `c` | char device  |
| `b` | block device |

::: tip
This is different from the [suffixes](./suffixes) after the file name, and
aligns more closely with `ls`.
:::

### Permissions (`perms`)

This fields renders permission bits as three triplets. The characters are color
coded using these standard colors.

| Value     | Color   |
| --------- | ------- |
| `r`       | yellow  |
| `w`       | red     |
| `x`       | green   |
| `t` / `T` | magenta |
| `s` / `S` | magenta |

### Owner (`user` and `group`)

`pls` dims owners other than the active user and groups that the active user
does not belong to.

::: warning
These columns are not available on Windows. Including them will have no effect.
:::

### Size (`size`)

`pls` displays the file size in human-readable units.

#### Preferences

**CLI flags:** `--units`/`-u`  
**Config YAML:** `units`

This is an [enum field](../reference/prefs.md#enums) with the following choices:

- `binary`: use binary units like kibibytes (KiB) and mibibytes (MiB) (default)

  ```shellsession
  $ pls -d size
  $ pls -d size -u binary
  ```

  ```yml
  prefs:
    units: binary
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">  Size       Name                   </span>
         <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
         <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
         <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
         <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
488<span style="color: #415f66; text-decoration-color: #415f66">  B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
245<span style="color: #415f66; text-decoration-color: #415f66">  B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
911<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   ﰌ   justfile               
 34<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>      LICENSE                
237<span style="color: #415f66; text-decoration-color: #415f66">  B</span>   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
 33<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  2<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
713<span style="color: #415f66; text-decoration-color: #415f66">  B</span>      pyproject.toml         
  7<span style="color: #415f66; text-decoration-color: #415f66">KiB</span>      README.md              
</code></pre>
</div>

- `decimal`: use decimal units like kilobytes (KB) and megabytes (MB)

  ```shellsession
  $ pls -d size -u decimal
  ```

  ```yml
  prefs:
    units: decimal
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline"> Size       Name                   </span>
        <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
        <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
        <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
        <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
488<span style="color: #415f66; text-decoration-color: #415f66"> B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
245<span style="color: #415f66; text-decoration-color: #415f66"> B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
911<span style="color: #415f66; text-decoration-color: #415f66"> B</span>   ﰌ   justfile               
 35<span style="color: #415f66; text-decoration-color: #415f66">KB</span>      LICENSE                
237<span style="color: #415f66; text-decoration-color: #415f66"> B</span>   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
 34<span style="color: #415f66; text-decoration-color: #415f66">KB</span>   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
  2<span style="color: #415f66; text-decoration-color: #415f66">KB</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
713<span style="color: #415f66; text-decoration-color: #415f66"> B</span>      pyproject.toml         
  7<span style="color: #415f66; text-decoration-color: #415f66">KB</span>      README.md              
</code></pre>
</div>

- `none`: show the exact number of bytes and no higher level units

  ```shellsession
  $ pls -d size -u none
  ```

  ```yml
  prefs:
    units: none
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">  Size       Name                   </span>
         <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
         <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
         <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
         <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
  488<span style="color: #415f66; text-decoration-color: #415f66">B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
  245<span style="color: #415f66; text-decoration-color: #415f66">B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
  911<span style="color: #415f66; text-decoration-color: #415f66">B</span>   ﰌ   justfile               
35149<span style="color: #415f66; text-decoration-color: #415f66">B</span>      LICENSE                
  237<span style="color: #415f66; text-decoration-color: #415f66">B</span>   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
34102<span style="color: #415f66; text-decoration-color: #415f66">B</span>   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
 1579<span style="color: #415f66; text-decoration-color: #415f66">B</span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
  713<span style="color: #415f66; text-decoration-color: #415f66">B</span>      pyproject.toml         
 7361<span style="color: #415f66; text-decoration-color: #415f66">B</span>      README.md              
</code></pre>
</div>

### Timestamps (`btime`, `ctime`, `mtime` and `atime`)

`pls` displays the timestamps in a human-readable format.

#### Preferences

**CLI flags:** `--time-fmt`/`-t`
**Config YAML:** `time_fmt`

This is a [string field](../reference/prefs.md#strings). It can take any textual
value.

- Default: shows the year and seconds as dimmed (as they are too broad and too
  narrow respectively to be useful).

  ```shellsession
  $ pls -d ctime -d mtime -d atime
  $ pls -d ctime -d mtime -d atime -t '[dim]%Y-[/]%m-%d %H:%M[dim]:%S[/] '
  ```

  ```yml
  prefs:
    time_fmt: "[dim]%Y-[/]%m-%d %H:%M[dim]:%S[/] "
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">Created at           Modified at          Accessed at                Name                   </span>
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 11:14<span style="color: #415f66; text-decoration-color: #415f66">:13 </span>   <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:41 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:33 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:33 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 23:41<span style="color: #415f66; text-decoration-color: #415f66">:10 </span>   <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:39 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span>   ﰌ   justfile               
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>      LICENSE                
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>   <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:08 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:08 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-13 17:55<span style="color: #415f66; text-decoration-color: #415f66">:43 </span>   <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-13 22:10<span style="color: #415f66; text-decoration-color: #415f66">:56 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:32 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-12 16:47<span style="color: #415f66; text-decoration-color: #415f66">:40 </span>     <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:42 </span>      pyproject.toml         
<span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 23:54<span style="color: #415f66; text-decoration-color: #415f66">:19 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 16:05<span style="color: #415f66; text-decoration-color: #415f66">:40 </span> <span style="color: #415f66; text-decoration-color: #415f66">2022-</span>02-14 23:46<span style="color: #415f66; text-decoration-color: #415f66">:17 </span>      README.md              
</code></pre>
</div>

- Custom: You can use
  [Python `strftime` codes](https://docs.python.org/3/library/datetime.html#strftime-and-strptime-format-codes)
  along with
  [Rich formatting markup](https://rich.readthedocs.io/en/latest/markup.html)
  and arbitrary text that is rendered as is.

  ```shellsession
  $ pls -d ctime -d mtime -t '[red]%Y[/]-[green]%m[/]-[blue]%d[/] %H:%M '
  ```

  ```yml
  prefs:
    time_fmt: "[red]%Y[/]-[green]%m[/]-[blue]%d[/] %H:%M "
  ```

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="text-decoration: underline">Created at        Modified at             Name                   </span>
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 11:14  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 11:14    <span style="color: #156667; text-decoration-color: #156667"></span>   <span style="color: #156667; text-decoration-color: #156667">dist/</span>                  
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 16:05  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 16:05    <span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">readme_assets</span><span style="color: #156667; text-decoration-color: #156667">/</span>         
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47    <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198; font-weight: bold">src</span><span style="color: #156667; text-decoration-color: #156667; font-weight: bold">/</span>                   
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 23:41  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 23:41    <span style="color: #2aa198; text-decoration-color: #2aa198">ﭧ</span>   <span style="color: #2aa198; text-decoration-color: #2aa198">tests</span><span style="color: #156667; text-decoration-color: #156667">/</span>                 
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47      <span style="color: #415f66; text-decoration-color: #415f66">.</span>flake8                 
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47      <span style="color: #415f66; text-decoration-color: #415f66">.</span>gitignore              
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47    ﰌ   justfile               
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47       LICENSE                
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47    <span style="font-style: italic"></span>  <span style="color: #415f66; text-decoration-color: #415f66; font-style: italic">.</span><span style="font-style: italic">pls.yml</span>                
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">13</span> 17:55  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">13</span> 17:55    <span style="color: #415f66; text-decoration-color: #415f66"></span>   <span style="color: #415f66; text-decoration-color: #415f66">poetry.lock</span>            
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">13</span> 22:10  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">12</span> 16:47      <span style="color: #415f66; text-decoration-color: #415f66">.</span>pre-commit-config.yaml 
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 16:05  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 16:05       pyproject.toml         
<span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 23:54  <span style="color: #dc322f; text-decoration-color: #dc322f">2022</span>-<span style="color: #859900; text-decoration-color: #859900">02</span>-<span style="color: #268bd2; text-decoration-color: #268bd2">14</span> 16:05       README.md              
</code></pre>
</div>

::: tip
Since the date columns can get too close, leave a trailing space in the format.
:::

::: warning
The field `btime` referring to a file's creation/birth timestamp is only
available on macOS. Windows uses `ctime` as the creation timestamp and Linux
doesn't store it at all.

On other operating systems, including it will have no effect.
:::

### Git status (`git`)

For all files (and some folders) `pls` shows the Git status as a two-letter
code. Refer to
[the `git-status` documentation](https://git-scm.com/docs/git-status#_output) to
understand the interpretation of this code.

::: warning
This column only appears when the following conditions are met.

- Git is installed on the system.
- The directory lies inside a Git repository.

In all other cases, including it will have no effect.
:::

::: warning
Older versions of Git seem to inconsistently quote files on Windows. If you run
into issues with Git, try updating your Git binaries.
:::
