# Functions

Functions are reusable procedures (custom blocks) that can return values, including 
primitives or structs. Functions always run in **Run without screen refresh** mode and 
**must only be called** from other **Run without screen refresh** procedures or 
functions to prevent undefined behavior.

Each function must **end with a `return` statement**. Using `stop_this_script` inside 
a function is undefined behavior.


## Declaring a Function

Use the `func` keyword to define a function. Optionally, include a return type for 
functions that return a struct.

```goboscript
func my_function(x, y) {
    return $x + $y;
}
```

```goboscript
func my_function(x, y) MyStruct {
    return MyStruct { ... };
}
```

---

## Returning Struct Variables

Functions can return struct variables by specifying the struct type as the return type.

### Basic Struct Return Example

```goboscript
struct Vector {
    x,
    y
}

func vec_add(Vector lhs, Vector rhs) Vector {
    return Vector {
        x: $lhs.x + $rhs.x,
        y: $lhs.y + $rhs.y
    };
}
```

### Using the Returned Struct

```goboscript
# Create vectors
Vector vec1 = Vector { x: 10, y: 20 };
Vector vec2 = Vector { x: 5, y: 15 };

# Call function that returns a struct
Vector result = vec_add(vec1, vec2);

# Access the returned struct's fields
say result.x; # Outputs: 15
say result.y; # Outputs: 35
```

!!!NOTE
    When returning struct variables from functions, the return type must be explicitly
    specified when returning a struct
---

## Default Argument Values

Function parameters can have **default values**, allowing callers to omit them:

```goboscript
func greet(name = "world") {
    return "Hello, " & $name & "!";
}
```

* `greet()` returns `"Hello, world!"`
* `greet("aspizu")` returns `"Hello, aspizu!"`

---

## Calling a Function

Functions are called by name with argument values:

```goboscript
say my_function(1, 2);
```

---

## Keyword Arguments

You can also call functions using **keyword arguments**, which specify parameter names
 explicitly. This is useful when using default arguments or calling functions with many 
 parameters:

```goboscript
greet(name: "aspizu")
```

This behaves the same as `greet("aspizu")`, but makes the call more readable—especially 
when multiple parameters are involved:

```goboscript
func introduce(name, title = "developer", location = "unknown") {
    return $name & " is a " & $title & " from " & $location;
}
```

Call it with keyword arguments:

```goboscript
introduce(name: "aspizu", location: "India")
# Equivalent to: introduce("aspizu", "developer", "India")
```

!!!NOTE 
    Keyword arguments can be used in any order, as long as the required parameters
    are provided:

    ```goboscript
    introduce(location: "Berlin", name: "Kai");
    # Still valid
    ```
