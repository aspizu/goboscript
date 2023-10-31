# Hat Blocks

### when flag clicked

```goboscript
onflag {}
```

```blocks
when gf clicked
```

### when key pressed

The key name must be written as a string.

```goboscript
onkey "space" {}
```

```blocks
when [space v] key pressed
```

### when this sprite clicked

```goboscript
onclick {}
```

```blocks
when this sprite clicked
```

### when backdrop switches to

The name of the backdrop must be written as a string.

```goboscript
onbackdrop "backdrop1" {}
```

```blocks
when backdrop switches to [backdrop1 v]
```

### when loudness/timer greater than

These hat blocks accept expressions as their operands.

```goboscript
onloudness 10 {}
ontimer 10 {}
```

```blocks
when [loudness v]> [10]

when [timer v] > [10]
```

### when i receive

The name of the message must be written as a string.

```goboscript
on "message1" {}
```

```blocks
when i receive [message1 v]
```

### when i start as a clone

```goboscript
onclone {}
```

```blocks
when i start as a clone
```
