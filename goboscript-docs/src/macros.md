# Macros

Macros are powerful tools to generate Scratch blocks.

```goboscript
macro macro_name!(arg1, arg2, arg3) -> expression;
```

Inside the definition of a macro, use arguments using the syntax `argument!`.
Using a macro inside it's definition is not allowed.

## Example

Let's create a macro to calculate the distance between two points.

```goboscript
macro distance!(x1, y1, x2, y2) -> sqrt((x2!-x1!)*(x2!-x1!) + (y2!-y1!)*(y2!-y1!));
```

When-ever this macro is used, goboscript will replace the blocks given to the macro
and copy-paste it.

```blocks
([sqrt v] of ((((...)-(...))*((...)-(...))) + (((...)-(...))*((...)-(...)))))
```

```goboscript
distance!(0, 0, 100, 100)
distance!(0, 0, mousex(), mousey())
```

```blocks
([sqrt v] of ((((100)-(0))*((100)-(0))) + (((100)-(0))*((100)-(0)))))
([sqrt v] of ((((mouse x)-(0))*((mouse x)-(0))) + (((mouse y)-(0))*((mouse y)-(0)))))
```

```admonish info title="Note"
Notice that the `mouse x` block has been duplicated in all places where it was used.
So, if a large expression is passed to a macro, it might generate huge blocks.
```

# Block Macros

Block macros are macros which generate statement blocks as opposed to reporter blocks.

```goboscript
macro macro_name! arg1, arg2, arg3 {
  // statements...
}
```

Block macros allow for powerful code generation, such as generating code for multiple
variables or lists, or wrapping custom blocks with additional behaviour (decorators).

## Example

Let's create a block macro to assign a variable with different values depending on
the result of some condition. In most programming languages, this exists as the ternary
operator. But as scratch does not have this, we can use block macros to implement
this.

```goboscript
macro ternary_assign! variable, condition, if_value, else_value {
  if condition! {
    variable! = if_value!;
  } else {
    variable! = else_value!;
  }
}
```

```goboscript
my_var = 1;
ternary_assign! my_var, my_var = 1, "True", "False";
```

```blocks
set [my_var v] to [1]
if <(my_var) = [1]> then
  set [my_var v] to [True]
else
  set [my_var v] to [False]
end
```

## Passing custom-blocks as arguments to macros

If a custom-block is to be passed as an argument to a macro, it must be used using the
syntax,

```goboscript
macro foo! func {
  call func!;
}
```

```goboscript
def myfunc {
  
}

foo! myfunc;
```
