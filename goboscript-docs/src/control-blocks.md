# Control Blocks

### if

```goboscript
if 1 = 1 {}
```

```blocks
if <[1] = [1]> then
```

### if-else

```goboscript
if 1 = 1 {} else {}
```

```blocks
if <[1] = [1]> then
else
```

### if-elif-else

goboscript will generate if-else blocks.

```goboscript
if $num = 1 {} elif $num = 2 {} elif $num = 3 {}
```

```blocks
if <(num:: custom) = [1]> then
else
  if <(num:: custom) = [2]> then
  else
    if <(num:: custom) = [3]> then
```

### repeat

```goboscript
repeat 10 {}
```

```blocks
repeat [10]
```

### repeat until

```goboscript
until var = "" {}
```

```blocks
repeat until <(var) = []>
```
