# Custom Blocks

Custom blocks are called procedures.

```goboscript
proc my_procedure arg1, arg2, type_name arg_name {
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

# Scratch Addons & Turbowarp Blocks

goboscript supports the Scratch Addons' debugger addon blocks, and TurboWarp blocks.

## `breakpoint`

The Scratch Addons' debugger addon block to set a breakpoint.

```goboscript
breakpoint;
```

## `log`

The Scratch Addons' debugger addon block to log a value.

```goboscript
log expr;
```
