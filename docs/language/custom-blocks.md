# Custom Blocks

Custom blocks are called procedures.

```goboscript
proc my_procedure arg1, arg2 {
    # code
}
```

## Arguments

Use arguments by prefixing `$` to the argument name.

```goboscript
proc my_procedure arg1, arg2 {
    say $arg1;
}
```

## Calling custom blocks

```goboscript
my_procedure arg1, arg2;
```
