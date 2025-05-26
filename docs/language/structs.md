# Structs

Structs are a way to group related variables or lists together.

## Declaration

```goboscript
struct my_struct {
    x,
    y,
    z
}
```

## Default values

Struct fields can have default values.

```goboscript
struct my_struct {
    field = "default_value"
}

my_struct v = my_struct {};
# -> v.field == "default_value"
```

## Usage

### Declaring a struct variable

```goboscript
my_struct my_variable = value;
```

### Declaring a struct list

```goboscript
list my_struct my_list;
```

## Accessing struct fields

### Accessing a struct variable field

```goboscript
my_variable.my_field
```

### Accessing a struct argument field

```goboscript
$my_argument.my_field
```

### Accessing a struct list field

```goboscript
my_list[index].my_field
```

## Struct literals

Struct literals are a way to create structs.

```goboscript
my_struct my_variable = my_struct {
    x: 10,
    y: 20,
    z: 30
};
```

## Passing structs

You can pass structs to procedures via arguments or variables.

```goboscript
list my_struct my_list;

proc my_procedure my_struct arg {
    # code
}

onflag {
    my_procedure my_struct {
        x: 10,
        y: 20,
        z: 30
    };
    my_procedure my_list[1];
}
```

```goboscript
onflag {
    my_struct foo = my_struct {
        x: 10,
        y: 20,
        z: 30
    };
    my_struct bar = foo;
}
```

# Limitations

## Variables

During struct variable assignment, fields are assigned sequentially. Consequently, if a 
field references another field of the same struct during assignment, it will use the 
updated value of previously assigned fields rather than their original values.

For example:

```goboscript
pair = Pair { left: 100, right: 200 };
pair = Pair { left: pair.left + 100, right: pair.left };
# Result: pair = Pair { left: 200, right: 200 };
```

In this case, the `pair.left` field is assigned first with the value 200, and 
subsequently `pair.right` references the updated value of `pair.left`.

This behavior can be mitigated by reordering the fields in the struct literal:

```goboscript
pair = Pair { right: pair.left, left: pair.left + 100 };
```

Same issues apply to all augmented assignment operators (`+=`)

## Lists

The issue with order of assignment applies to struct lists as well.
