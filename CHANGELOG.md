# Changelog

### 30th September 2026: Unused variable and list warnings [(#387)](https://github.com/aspizu/goboscript/pull/387)

The compiler reports unused variables and lists.

### 30th September 2026: Project notes in the Stage [(#386)](https://github.com/aspizu/goboscript/pull/386)

Embed a text file as a Stage comment by setting `notes` in `goboscript.toml`.

```toml
notes = "NOTES.md"
```

### 22nd August 2026: Ternary expressions [(#239)](https://github.com/aspizu/goboscript/pull/239)

Use `if (condition) true_value else false_value` inside expressions, including
nested ternaries.

```goboscript
say if (score > 100) "winner" else "loser";
```

### 15th August 2026: Generated Scratch script layout [(#366)](https://github.com/aspizu/goboscript/pull/366)

Generated scripts are arranged vertically with spacing based on their block sizes
and ordered by event type, making them easier to browse in Scratch.

### 1st August 2026: Fixed-length list initialization [(#363)](https://github.com/aspizu/goboscript/pull/363)

Initialize a list with a repeated value using `[value; length]`. This also works
with struct-typed lists. The maximum length is 200,000 items.

```goboscript
list scores = [0; 1000];
```

### 26th May 2026: `STRINGIFY` built-in macro [(#268)](https://github.com/aspizu/goboscript/pull/268)

```goboscript
STRINGIFY(hello world) # becomes "hello world"
```

### 9th May 2026: `show` and `hide` statements work with struct-typed lists and variables [(#284)](https://github.com/aspizu/goboscript/pull/284)

```goboscript
struct Point {x,y,z}
var Point p;
# these generate show/hide for each field in p
show p;
hide p;
```
