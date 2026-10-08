# Basic Examples

Let's start with simple examples of Goboscript syntax.

Try one example at a time in your sprite's `.gs` file, keeping the existing
`costumes` line.

```goboscript
# This is a single-line comment

# Numbers
var integer = 42;        # Integer
var float_num = 3.14;    # Float
var binary = 0b1010;     # Binary (value: 10)
var hex = 0xFF;          # Hexadecimal (value: 255)
var octal = 0o777;       # Octal (value: 511)

# Strings
var greeting = "Hello, World!";
var escaped = "Quotes: \"example\"";
var unicode = "\u1234";  # Unicode character

# Booleans
var flag = true;         # Will be compiled as 1
var not_flag = false;    # Will be compiled as 0
```

### Variables and Operators

```goboscript
onflag {
    # Variable declaration and assignment
    x = 10;
    y = 20;
    sum = x + y;         # Addition: 30
    diff = x - y;        # Subtraction: -10
    product = x * y;     # Multiplication: 200
    quotient = y / x;    # Division: 2
    floor_div = y // x;  # Floor division: 2
    remainder = y % x;   # Modulo: 0
    text = "Hello" & " World"; # String concatenation: "Hello World"

    # Compound assignment
    x += 5;                  # x = x + 5
    y -= 3;                  # y = y - 3
    product *= 2;            # product = product * 2
    quotient /= 4;           # quotient = quotient / 4
    text &= "!";             # text = text & "!"

    # Comparison operators
    is_equal = x == y;   # Equal to
    not_equal = x != y;  # Not equal to
    greater = x > y;     # Greater than
    less = x < y;        # Less than
    greater_equal = x >= y; # Greater than or equal to
    less_equal = x <= y;    # Less than or equal to

    # Logical operators
    both = x > 0 and y > 0;  # Logical AND
    either = x > 0 or y < 0; # Logical OR
    inverse = not (x > 0);   # Logical NOT
}
```

### Control Flow

```goboscript
onflag {
    x = 10;

    # Simple if statement
    if x > 5 {
        say "x is greater than 5";
    }

    # If-else statement
    if x % 2 == 0 {
        say "x is even";
    } else {
        say "x is odd";
    }

    # If-elif-else statement
    if x < 0 {
        say "x is negative";
    } elif x == 0 {
        say "x is zero";
    } else {
        say "x is positive";
    }

    # Boolean coercion
    if timer() {  # Equivalent to: if timer() == 1
        say "Timer equals 1";
    }
}
```

### Loops

```goboscript
onflag {
    # Repeat loop (fixed number of iterations)
    repeat 5 {
        say "Repeated message";
    }

    # Repeat with counter
    i = 1;
    repeat 10 {
        say "Iteration " & i;
        i++;
    }

    # Until loop (continues until condition is true)
    counter = 0;
    until counter > 5 {
        say "Counter: " & counter;
        counter++;
    }

    # Forever loop (infinite loop)
    forever {
        say "Press stop to exit";
        if key_pressed("space") {
            stop_this_script;
        }
    }
}
```

### Functions and Procedures

#### Procedures

```goboscript
# Define a procedure
proc greet_user {
    say "Hello, user!";
}

# Procedure with parameters
proc personalized_greeting name {
    say "Hello, " & $name & "!";
}

# Procedure with local variables
proc calculate_sum a, b {
    local result = $a + $b;
    say "Sum: " & result;
}

onflag {
    # Call procedures
    greet_user;
    personalized_greeting "John";
    calculate_sum 5, 10;
}
```

#### Functions

Functions are similar to procedures but return values.

The `greet_world` procedure runs without screen refresh, so it can call functions
safely.

```goboscript
# Define a function
func sum(x, y) {
    return $x + $y;
}

# Define a function returning a string
func create_greeting(name) {
    return "Hello, " & $name & "!";
}

proc greet_world {
    message = create_greeting("World");
    say message;  # Outputs: "Hello, World!"
}

onflag {
    greet_world;
}
```

### Data Structures

#### Lists

For the file-backed list, create `data.txt` in the project directory with one item
per line.

```goboscript
# Declare an empty list
list my_list;

# Load list from file
list data "data.txt";

onflag {
    # Add items to a list
    add "apple" to my_list;
    add "banana" to my_list;
    add "cherry" to my_list;

    # Access list items (1-indexed)
    first_item = my_list[1];  # "apple"

    # Replace items
    my_list[2] = "blueberry";     # Replace "banana" with "blueberry"

    # Insert at position
    insert "apricot" at my_list[1]; # Insert at the beginning

    # Delete an item
    delete my_list[3];

    # Check length
    size = length my_list;

    # Check if an item is in the list
    if "cherry" in my_list {
        say "Found cherry!";
    }

    # Get random item
    random_fruit = my_list["random"];

    # Clear the list
    delete my_list;
}
```

#### Structures and Enums

```goboscript
# Define a struct
struct Point {
    x,
    y
}

# Define an enum
enum Direction {
    North,  # 0
    East,   # 1
    South,  # 2
    West    # 3
}

# Enum with explicit values
enum Color {
    Red = "red",
    Green = "green",
    Blue = "blue"
}

onflag {
    # Create a struct instance
    Point p = Point { x: 10, y: 20 };

    # Access struct fields
    say "Coordinates: " & p.x & ", " & p.y;

    # Use enum values
    current_direction = Direction.North;
    if current_direction == Direction.North {
        say "Heading north";
    }

    say "Selected color: " & Color.Red;
}
```
