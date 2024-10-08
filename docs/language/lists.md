# Lists

The same rules apply to lists as they do to variables. You can declare a list for all sprites in `stage.gs`, for this sprite only in the sprite's `.gs` file. There are no
local lists.

## Delete all items from a list

This statement is considered the declaration of a list.

```goboscript
delete list;
```

## Delete item at index from a list

```goboscript
delete list[index];
```

### Delete last item of list

```goboscript
delete list["last"];
```

## Add item to list

```goboscript
add item to list;
```

## Insert item at index in list

```goboscript
insert item at list[index];
```

## Replace item at index in list

```goboscript
list[index] = item;
```

## Apply operator to item at index in list

```goboscript
list[index] += 1;
list[index] -= 1;
list[index] *= 1;
list[index] /= 1;
list[index] // = 1; # Floor Division
list[index] %= 1;
list[index] &= "suffix";
```

## Get item at index from list

```goboscript
say list[index];
```

### Get last item of list

```goboscript
say list["last"];
```

## Get length of list

```goboscript
say length list;
```

## Get index of item in list

TODO

## Check if list contains item

```goboscript
say item in list;
```

## Show list monitor

```goboscript
show list;
```

## Hide list monitor

```goboscript
hide list;
```
