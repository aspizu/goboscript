# Syntax

goboscript has a syntax inspired from languages such as C, Rust, and Python.

## Comments

```goboscript
# single-line comments are the only option.
```

## Whitespace & Indentation

Whitespace & indentation has no semantic significance.

## Numbers

```goboscript
0b111 # Binary:      8
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

## Operators

| Operator | Description               |
| -------- | ------------------------- |
| `+`      | Addition                  |
| `-`      | Subtraction / Unary Minus |
| `*`      | Multiplication            |
| `/`      | Division                  |
| `//`     | Floor Division            |
| `%`      | Modulo                    |
| `&`      | Join                      |
| `==`     | Equal                     |
| `!=`     | Not equal                 |
| `<`      | Less than                 |
| `<=`     | Less than or equal to     |
| `>`      | Greater than              |
| `>=`     | Greater than or equal to  |
| `in`     | Contains                  |
| `&`      | Join                      |
| `not`    | Not                       |
| `and`    | And                       |
| `or`     | Or                        |

### Mathematical operators

| Operator  | Description       |
| --------- | ----------------- |
| `round`   | Round             |
| `abs`     | Absolute value    |
| `floor`   | Floor             |
| `ceil`    | Ceil              |
| `sqrt`    | Square root       |
| `sin`     | Sine              |
| `cos`     | Cosine            |
| `tan`     | Tangent           |
| `asin`    | Arc sine          |
| `acos`    | Arc cosine        |
| `atan`    | Arc tangent       |
| `ln`      | Natural logarithm |
| `log`     | Logarithm         |
| `antiln`  | e ^               |
| `antilog` | 10 ^              |
