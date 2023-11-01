# Variables

To define a variable, first assign to it.
```goboscript
variable = 10;
```

```blocks
set [variable v] to [10]
```

goboscript provides compound-assignment operators.

```goboscript
variable++;
variable += 2;
variable -= 2;
variable *= 2;
variable /= 2;
variable %= 2;
variable &= ".";
```

```blocks
change [variable v] by [1]
change [variable v] by [2]
change [variable v] by [-2]
set [variable v] to ((variable) * [2])
set [variable v] to ((variable) / [2])
set [variable v] to ((variable) mod [2])
set [variable v] to (join (variable) [.])
```

To use a variable, refer to it by its name.

```goboscript
say variable;
```

```blocks
say (variable)
```

## Global Variables

Variables by default are "For this sprite only". To make a variable "For all sprites",
there are two ways.

Either add the variable name in a `variables` declaration in `stage.gobo`.

```goboscript
variables global_variable;
```

or assign to the variable in `stage.gobo`.

```goboscript
onflag {
  global_variable = 0;
}
```

## Cloud Variables

To define a cloud variable declare them in the stage.

```goboscript
cloud my_cloud_variable1, my_cloud_variable2;
```
