# Standard Library

The standard library is a collection of useful procedures, functions and macros that
can be used in any goboscript project.

## Standard library headers

| Header | Description |
|--------|-------------|
| `std/math` | Mathematical operations |
| `std/string` | String manipulation |
| `std/algo` | Various algorithms |
| `std/emoji` | Emoji database |

Include a header using the `%include` directive.

```goboscript
%include std/math
```

goboscript's dead code elimination will remove any unused procedures and functions from
the compiled project.
