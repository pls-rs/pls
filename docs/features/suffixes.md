---
lang: en-GB
title: Suffixes
description: >-
  pls follows file and directory names with suffixes that provide more 
  information about its type.
---

# Suffixes

`pls` adds suffixes behind anything other than a regular UNIX file. This helps
to identify the nature of the file and provide more information about it.

<div
    style="background-color: #002b36; color: #839496;"
    class="language-">
  <pre style="color: inherit;"><code style="color: inherit;"><span style="color: #2aa198; text-decoration-color: #2aa198"></span>   <span style="color: #2aa198; text-decoration-color: #2aa198">dir</span><span style="color: #156667; text-decoration-color: #156667">/</span>                        
    fifo<span style="color: #415f66; text-decoration-color: #415f66">|</span>                       
    file                        
    sock<span style="color: #415f66; text-decoration-color: #415f66">=</span>                       
    sym_broken<span style="color: #415f66; text-decoration-color: #415f66">@ →</span> <span style="color: #dc322f; text-decoration-color: #dc322f">none⚠</span>         
    sym_dir<span style="color: #415f66; text-decoration-color: #415f66">@ →</span> <span style="color: #2aa198; text-decoration-color: #2aa198">dir</span><span style="color: #156667; text-decoration-color: #156667">/</span>             
    sym_self<span style="color: #415f66; text-decoration-color: #415f66">@ ↺</span> <span style="color: #dc322f; text-decoration-color: #dc322f">sym_self</span>        
    sym_sym<span style="color: #415f66; text-decoration-color: #415f66">@ →</span> sym_dir<span style="color: #415f66; text-decoration-color: #415f66">@ →</span> <span style="color: #2aa198; text-decoration-color: #2aa198">dir</span><span style="color: #156667; text-decoration-color: #156667">/</span>  
</code></pre>
</div>

## Reference

`pls` can identify and annotate the following file types.

| Type              | Suffix              |
| ----------------- | ------------------- |
| directory         | `/`                 |
| named pipe / FIFO | <code>&vert;</code> |
| socket            | `=`                 |
| symlink           | `@`                 |

Symlinks, being special, have additional information in the suffix.

- Normally symlinks have an arrow `→` pointing to their destination node, as with
  `sym_dir` in the example above.
- If their destination node is a symlink, it is suffixed in the same way,
  forming a chain, as with `sym_sym` in the example above.
- If the destination node does not exist, the link is displayed in red with an
  error sign (`⚠`).
- If the symlinks form a loop, the link is displayed in red and the arrow is
  replaced with the loop symbol (`↺`).
