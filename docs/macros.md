Macros in GoboScript
====================

Defining Macros
---------------

Macros can be defined at the top-level by using a `macro` statement.

```
macro NAME <ARGUMENTS, ...> -> <EXPRESSION>;
```

Using Macros
------------

The syntax to use a macro is similar to the syntax of reporter blocks but the name must
be prefixed with a `!` symbol.

```
!NAME(<ARGUMENTS>, ...)
```

During compilation, this will be replaced with the body of the macro, substituting the
given arguments.

Examples
========
