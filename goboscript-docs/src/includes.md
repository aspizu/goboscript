# Includes

goboscript allows for including code from multiple files in a single sprite.

```goboscript
%use "path-to-file.h.gobo";
```

```admonish note
The file extension `.h.gobo` is used to tell the compiler that it is a part of some
other sprite, and should not be treated as a sprite.

Another way to prevent files from becoming sprites is to place them inside a directory.
Putting `include.gobo` inside `includes/` and using `%use "includes/include.gobo"` would
work aswell.
```
