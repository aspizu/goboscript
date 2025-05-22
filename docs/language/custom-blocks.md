# Custom Blocks

Custom blocks, also known as **procedures** can take input arguments, but unlike
functions, they do **not return values**.

## Declaring a Custom Block

Use the `proc` keyword to define a custom block. List argument names separated by 
commas.

```goboscript
proc my_procedure arg1, arg2 {
    say $arg1;
    say $arg2;
}
```

Use the `nowarp` keyword before `proc` to make the custom block
*run without screen refresh* **unchecked**.

```goboscript
nowarp proc my_procedure arg1, arg2 {
    say $arg1;
    say $arg2;
}
```

## Struct-Typed Arguments

You can take in struct values by specifying the type name before the argument name.

```goboscript
proc process_item Item item_data {
    say $item_data.name;
}
```

---

## Default Argument Values

Just like functions, **procedures support default argument values**. This allows a 
caller to skip certain arguments when calling the block.

```goboscript
proc greet name = "world" {
    say "Hello, " & $name & "!";
}
```

* `greet` → says "Hello, world!"
* `greet "aspizu"` → says "Hello, aspizu!"

---

## Keyword Arguments

Procedures can also be called using **keyword arguments**, specifying each parameter by 
name. This improves readability, especially when not all parameters are passed or when 
calling with many arguments.

```goboscript
proc introduce name, title = "developer", location = "unknown" {
    say $name & " is a " & $title & " from " & $location;
}
```

Call it using keyword arguments:

```goboscript
introduce name: "aspizu", location: "India";
# Output: "aspizu is a developer from India"
```

Keyword arguments can be **used in any order**, as long as required arguments are 
provided:

```goboscript
introduce location: "Berlin", name: "Kai";
# Output: "Kai is a developer from Berlin"
```

---

## Calling Custom Blocks

Call a procedure using positional or keyword arguments:

```goboscript
# Positional
my_procedure "hello", 3;

# Keyword
my_procedure arg2: 3, arg1: "hello";
```

Use `$argname` inside the block to access the arguments.
