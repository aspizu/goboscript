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
