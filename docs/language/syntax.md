# Syntax

goboscript has a syntax inspired from languages such as C, Rust, and Python. Whitespace
and indentation has no semantic significance. Statements end in semicolons.

## Comments

```goboscript
# single-line comments are the only option.
```

## Numbers

```goboscript
0b111 # Binary:      7
0xFF  # Hexadecimal: 255
0o777 # Octal:       511
1024  # Decimal:     1024
3.141 # Float:       3.141
```

## Strings

```goboscript
"Hello, World!"
"Hello, \"World\"!"
"\u1234" # Unicode escape
"\n" # Newline
"\t" # Tab
```

## Booleans

`true` is replaced with `1` during compilation.

`false` is replaced with `0` during compilation.

## Boolean Coercion

Scratch does not allow placing a non-boolean block inside a boolean input. (With the
exception of **item # of list** and **item of list**). goboscript will automatically
coerce non-boolean blocks into conditions by wrapping them in a comparison with `1`.

```goboscript
if timer() {
    ...
}
```

is equivalent to

```goboscript
if timer() == 1 {
    ...
}
```
