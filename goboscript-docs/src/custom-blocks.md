# Custom Blocks

To define a custom block,

```goboscript
def custom_block arg1, arg2, arg3 {}
```

To define a custom block with the "Run without screen refresh" **unchecked**.

```goboscript
nowarp def custom_block arg1, arg2, arg3 {}
```

```blocks
define custom_block (arg1) (arg2) (arg3)
...
```

## Arguments

goboscript does not support boolean arguments, use boolean coercing and the `true` and
`false` constants.

To refer to a custom block's arguments use the `$argument` syntax.

## Using custom blocks

```gsc
custom_block "one", "two", "three";
```

```blocks
custom_block [one] [two] [three]:: custom
```

# Local Variables

goboscript allows for custom block scoped local variables using the `local` keyword.

```goboscript
def foo {
  local var = 1;
  say var;
}
```

```blocks
define foo
  set [foo:var v] to [1]
  say (foo:var)
```

```admonish warning
Remember that these are just regular "For this sprite only" variables. Using them with
recursion will have unexpected results.
```
