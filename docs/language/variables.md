# Variables

## For all sprites variables

To define a for all sprites variable, either assign to it, or set a initial value
in stage. All variables defined in the stage are for all sprites.


```{.goboscript title="Assign to the variable" }
onflag {
    global_variable = 100 + 200;
}
```

```{.goboscript title="Set a initial value" }
global_variable = 100;
```

## For this sprite only variables

To define a for this sprite only variable, either assign to it, or set a initial value
in the sprite.

```{.goboscript title="Assign to the variable" }
onflag {
    this_sprite_variable = 100 + 200;
}
```

```{.goboscript title="Set a initial value" }
this_sprite_variable = 100;
```

## Local variables

Local variables are defined in a procedure (custom block) and are only accessible
inside the procedure.

```goboscript
def my_procedure {
    local local_variable = 100;
}
```

If a local variable is defined with the same name as a regular variable, that variable
will be shadowed. 

## Show or hide variable monitors

To show or hide a variable monitor, use the `show` or `hide` statement.

```{.goboscript title="Show a variable monitor" }
show global_variable;
```

```{.goboscript title="Hide a variable monitor" }
hide global_variable;
```

## Set or change variable

Set variables using the syntax:

```goboscript
variable = expression;
```

Change variables using the syntax:

```goboscript
variable += expression;
```

### Compound assignment operators

```goboscript
variable -= expression; # Will use the change variable statement.
variable *= expression; # Rest use the set variable statement.
variable /= expression;
variable %= expression;
variable &= expression; # & is the join operator.
```

```scratchblocks
change [variable v] by ((0) - (expression))
```
