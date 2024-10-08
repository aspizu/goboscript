# Variables

## Variables for all sprites

To declare a variable for all sprites, assign to it in `stage.gs`.

```goboscript
onflag {
    pi = 3.14159265359;
}
```

## Variables for this sprite only

To declare a variable for this sprite only, assign to it in the sprite's `.gs` file.

```goboscript
onflag {
    x = 0;
}
```

## Local variables (for a procedure only)

Local variables is a feature of goboscript, which lets you define a variable which can
only be used inside a procedure and is not accessible outside of it.

```goboscript
proc my_procedure {
    local x = 0;
    x = x + 1;
}
```

In the compiled Scratch project, the variable `x` will be named as `my_procedure.x`.

!!! note
    Local variables will have unexpected behavior if the procedure is recursive.

## Set variable

```goboscript
x = 10;
```

## Change variable

```goboscript
x += 1;
```

## Change variables using a operator

```goboscript
x += 1;
x -= 1;
x *= 2;
x /= 2;
x //= 2; # Floor Division
x %= 2;
x &= "str";
x++; # Increment by 1
x--; # Decrement by 1
```

The `-=` statement is implemented using the change variable block.
