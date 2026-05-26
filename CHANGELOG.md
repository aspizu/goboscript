# Changelog

### 9th May 2026: `show` and `hide` statements work with struct-typed lists and variables [(#284)](https://github.com/aspizu/goboscript/pull/284)

```goboscript
struct Point {x,y,z}
var Point p;
# these generate show/hide for each field in p
show p;
hide p;
```
