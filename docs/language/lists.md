# Lists

The same rules apply for lists as for variables regarding **for all sprites** and
**for this sprite only**.

## Declaration

```goboscript
list list_name;
```

```goboscript
list type_name list_name;
```

## Operations

### Add item to list

```goboscript
add value to list_name;
```

### Delete item from list at index

```goboscript
delete list_name[index];
```

### Delete all items from list

```goboscript
delete list_name;
```

### Insert item at index in list

```goboscript
insert value at list_name[index];
```

### Replace item at index in list

```goboscript
list_name[index] = value;
```

### Get item at index in list

```goboscript
value = list_name[index];
```

### Get index of item in list

TODO

### Get length of list

```goboscript
len = length list_name;
```

### Check if item is in list

```goboscript
if value in list_name {
    ...
}
```

### Show/Hide List Monitor

```goboscript
show list_name;
```

```goboscript
hide list_name;
```

### Get random/last item in list

```goboscript
value = list_name["random"];
```

```goboscript
value = list_name["last"];
```

## Compound Assignment

| Operator               | Implementation                                 |
|------------------------|------------------------------------------------|
| `list_name[index]++;`  | ![](../assets/list_increment.png){width="400"} |
| `list_name[index]--;`  | ![](../assets/list_decrement.png){width="400"} |
| `list_name[index] += y;` | ![](../assets/list_add.png){width="400"} |
| `list_name[index] -= y;` | ![](../assets/list_subtract.png){width="400"} |
| `list_name[index] *= y;` | ![](../assets/list_multiply.png){width="400"} |
| `list_name[index] /= y;` | ![](../assets/list_divide.png){width="400"} |
| `list_name[index] //= y;` | ![](../assets/list_floor_divide.png){width="400"} |
| `list_name[index] %= y;` | ![](../assets/list_mod.png){width="400"} |
| `list_name[index] &= y;` | ![](../assets/list_join.png){width="400"} |

## List Data

Initial data for lists can be stored inside the project. This behaves the same way
loading text files into lists works in the Scratch editor. In addition to loading text 
files, you can also load data from various scripts and commands. This is useful for
creating look-up tables or loading data from images or videos.


### Loading data from text files

Each line in the text file will be added to the list as a separate item.

```goboscript
list list_name = file ```path/to/file.txt```;
```

### Loading data from bash script

The bash script enclosed in triple-backticks will be executed, and the standard output
will be stored in the list, one item per line. The working directory will be set to the 
project directory.

```goboscript
list list_name = ```cat path/to/file.txt```;
```

### Loading data from any other program

The name of the program may be specified before the triple-backticks. This program will
be executed with the standard input set to the script enclosed in the triple-backticks.
The standard output will be stored in the list, one item per line. The working directory
will be set to the project directory.

Any program that accepts input from stdin and outputs to stdout can be used.

For example, to load data from a python script:

```goboscript
list list_name = python ```
import random
for _ in range(5):
    print(random.randint(-5, 5))
```;
```

!!! tip
    If your script takes a long time to run, you can use bash to cache the output of the
    script.

    Let's say that your script converts a file `DEPENDENCY.txt`. You wish to only
    re-run the script if the file `DEPENDENCY.txt` has changed. We can use stat
    to get the last modification time of the file.

    ```bash
    TIME=$(stat -c %Y DEPENDENCY.txt)
    if [ $TIME -eq $(< DEPENDENCY.time)]; then
        cat DEPENDENCY.cached
        exit
    fi
    echo $TIME > DEPENDENCY.time
    python convert_file.py DEPENDENCY.txt | tee DEPENDENCY.cached
    ```

### Struct List Data

If the list's type is a struct, each field will be filled with the value of the
corresponding line in the data.

Example:

```goboscript
struct vec3d { x, y, z };
list vec3d points = file ```file.txt```;
```

contents of `file.txt`:
```
10
20
30
40
50
60
```

resulting in the following list:

```goboscript
[
    vec3d {
        x: 10,
        y: 20,
        z: 30
    },
    vec3d {
        x: 40,
        y: 50,
        z: 60
    }
]
```
