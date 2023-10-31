# Expressions

## Values

```goboscript
say "This is a \"text\" value";
```

```blocks
say [This is a "text" value]
```

```goboscript
say 100;
say -3.14159;
```

```blocks
say [100]
say [-3.14159]
```

```goboscript
say false;
say true;
```

```blocks
say [0]
say [1]
```

## Arithmetic Operators

```goboscript
10 + 20
10 - 20
10 * 20
10 / 20
10 % 20
```

```blocks
([10] + [20])
([10] - [20])
([10] * [20])
([10] / [20])
([10] mod [20])
```

## Join Operator

```goboscript
"apple " & "banana"
```

```blocks
(join [apple ] [banana])
```

## Comparison Operators

```goboscript
"a" in "apple"
"a" = "A"
2 < 3
2 > 1
```

```blocks
<[apple] contains [a]?>
<[a] = [A]>
<[2] \< [3]>
<[2] \> [1]>
```

goboscript provides additional operators which get compiled into workarounds.

```goboscript
"a" != "b"
2 >= 2
3 <= 3
```

```blocks
<not <[a] = [b]>>
<not <[2] \< [2]>>
<not <[3] \> [3]>>
```

## Logical Operators

```goboscript
0 < var and var < 5
var = 1 or var = 2
not "a" = "b"
```

```blocks
<<[0] \< (var)> and <(var) \< [5]>>
<<(var) = [1]> or <(var) = [2]>>
<not <[a] = [b]>>
```

# Precedence of operators

All binary operators are left to right associative.

This means that 
```goboscript
1 + 2 + 3 + 4
```
becomes
```blocks
((([1] + [2]) + [3]) + [4])
```

| Operators in order of precedence             |
|----------------------------------------------|
| `and`, `or`                                  |
| `not`, `in`, `=`, `!=`, `<`, `>`, `<=`, `>=` |
| `&`, `+`, `-`                                |
| `*`, `/`, `%`                                |
| unary `-`, `(` expression `)`                |

# Boolean Coercing

Scratch does not allow putting a round block (Value reporter) inside a angled hole
(Expects boolean).

goboscript will workaround this.

```goboscript
variable = true;
if variable {}
```

```blocks
set [variable v] to [1]
if <not <(variable) = [0]>> then
```
