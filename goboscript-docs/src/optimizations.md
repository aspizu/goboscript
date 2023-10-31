# Optimizations

goboscript applies various optimizations to code.

# Constant Folding

<https://en.wikipedia.org/wiki/Constant_folding>

# Unnecessary use of the `not` operator

`not not a` will be turned into `a`.

This is useful when using boolean-coercing, `if not a {}` will become `if a = 0 {}`

# De-Morgan's Law

<https://en.wikipedia.org/wiki/De_Morgan's_laws>

`not a and not b` will be turned into `not (a or b)`

`not a or not b` will be turned into `not (a and b)`
