# Operators

## Unary Operators

| Operator | Implementation |
|----------|----------------|
| `not x == true` | ![](../assets/not.png){width="125"} |
| `length x` | ![](../assets/length.png){width="125"} |
| `round x` | ![](../assets/round.png){width="125"} |
| `abs x` | ![](../assets/abs.png){width="125"} |
| `floor x` | ![](../assets/floor.png){width="125"} |
| `ceil x` | ![](../assets/ceil.png){width="125"} |
| `sqrt x` | ![](../assets/sqrt.png){width="125"} |
| `sin x` | ![](../assets/sin.png){width="125"} |
| `cos x` | ![](../assets/cos.png){width="125"} |
| `tan x` | ![](../assets/tan.png){width="125"} |
| `asin x` | ![](../assets/asin.png){width="125"} |
| `acos x` | ![](../assets/acos.png){width="125"} |
| `atan x` | ![](../assets/atan.png){width="125"} |
| `ln x` | ![](../assets/ln.png){width="125"} |
| `log x` | ![](../assets/log.png){width="125"} |
| `antiln x` | ![](../assets/antiln.png){width="125"} |
| `antilog x` | ![](../assets/antilog.png){width="125"} |

## Binary Operators

| Operator | Implementation |
|----------|----------------|
| `x + y` | ![](../assets/add.png){width="125"} |
| `x - y` | ![](../assets/subtract.png){width="125"} |
| `x * y` | ![](../assets/multiply.png){width="125"} |
| `x / y` | ![](../assets/divide.png){width="125"} |
| `x // y` | ![](../assets/floor_divide.png){width="200"} |
| `x % y` | ![](../assets/mod.png){width="125"} |
| `x & y` | ![](../assets/join.png){width="125"} |
| `x == y` | ![](../assets/eq.png){width="125"} |
| `x != y` | ![](../assets/neq.png){width="125"} |
| `x < y` | ![](../assets/lt.png){width="125"} |
| `x <= y` | ![](../assets/leq.png){width="125"} |
| `x > y` | ![](../assets/gt.png){width="125"} |
| `x >= y` | ![](../assets/geq.png){width="125"} |
| `x == true and y == true`| ![](../assets/and.png){width="400"} |
| `x == true or y == true`| ![](../assets/or.png){width="400"} |
| `x in y` | ![](../assets/in.png){width="125"} |
| `x not in y` | ![](../assets/notin.png){width="125"} |
| `x[y]` | ![](../assets/letterof.png){width="125"} |

## Operator Precedence

Highest precedence first:

| Order | Operators | Grouping |
| --- | --- | --- |
| 1 | Calls `f(...)`, indexing `x[i]`, and field access `x.field` | Left to right |
| 2 | Unary operators, including `-`, `not`, and `length` | Right to left |
| 3 | `*`, `/`, `//`, `%` | Left to right |
| 4 | `+`, `-` | Left to right |
| 5 | `<`, `<=`, `>`, `>=` | Left to right |
| 6 | `&` | Right to left |
| 7 | `in`, `not in`, `==`, `!=` | Left to right |
| 8 | `and` | Left to right |
| 9 | `or` | Left to right |

## Reporters

### pick random () to ()

```goboscript
random(x, y)
```

```scratchblocks
(pick random (x) to (y))
```

### () contains ()

```goboscript
contains(string, substring)
```

```scratchblocks
<(string) contains (substring)?>
```
