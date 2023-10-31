# Lists

To define a list, first assign to it.

```goboscript
list = [];
```

```blocks
delete all of [list v]
```

## add to list

```goboscript
list.add "thing";
```

```blocks
add [thing] to [list v]
```

goboscript will transform list assignments with values into add to list blocks.

```goboscript
list = [1, 2, 3];
```

```blocks
delete all of [list v]
add [1] to [list v]
add [2] to [list v]
add [3] to [list v]
```

## delete of list

```goboscript
list.delete 1;
```

```blocks
delete [1] of [list v]
```

## insert at list

```goboscript
list.insert 1, "thing";
```

```blocks
insert [thing] at [1] of [list v]
```

## replace item of with

```goboscript
list[1] = "thing";
```

```blocks
replace item [1] of [list v] with [thing]
```

goboscript provides compound-assignment operators.

```goboscript
list[1] += 2;
list[1] -= 2;
list[1] *= 2;
list[1] /= 2;
list[1] %= 2;
list[1] &= ".";
```

```blocks
replace item [1] of [list v] with ((item [1] of [list v]) + [2])
replace item [1] of [list v] with ((item [1] of [list v]) - [2])
replace item [1] of [list v] with ((item [1] of [list v]) * [2])
replace item [1] of [list v] with ((item [1] of [list v]) / [2])
replace item [1] of [list v] with ((item [1] of [list v]) mod [2])
replace item [1] of [list v] with (join (item [1] of [list v]) [.])
```

## item of list

```goboscript
list[1]
```

```blocks
(item [1] of [list v])
```

## item # of list

```goboscript
list.index("thing")
```

```blocks
(item # of [thing] in [list v])
```

## length of list

```goboscript
list.length
```

```blocks
(length of [list v])
```

## list contains

```goboscript
"thing" in list
```

```blocks
<[list v] contains [thing] ?>
```

## show and hide

```goboscript
list.show;
list.hide;
```

```blocks
show list [list v]
hide list [list v]
```

## Global Lists

Lists by default are "For this sprite only". To make a list "For all sprites",
there are two ways.

Either add the variable name in a `lists` declaration in `stage.gobo`.

```goboscript
lists global_list;
```

or assign to the list in `stage.gobo`.

```goboscript
onflag {
  global_list = [];
}
```
