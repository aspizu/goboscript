GoboScript Documentation
========================

* [Getting Started](#getting-started)
* [Syntax](#syntax)

Getting Started
===============

Follow the instructions in [README.md](/README.md) to install the GoboScript compiler
`gsc`.

Optionally, install the [VSCode Extension](/vscode-goboscript/README.md) to get syntax
highlighting and other language features for `.gs` files.

Run `gsc examples/helloworld examples/helloworld/build.sb3` to compile the example
[helloworld](/examples/helloworld) project.

Open the generated `build.sb3` file in the
[Scratch Editor](https://scratch.mit.edu/projects/editor) or
[Turbowarp Editor](https://turbowarp.org/editor).

Syntax
======

Text inside `< ... >` are used to specify grammar. They are not actually a part of the
syntax.

Comments
--------

Only multi-line comments are supported. 
### Syntax
```c
/* This is a multi-line comment */
```
These comments cannot be nested.

Literals
--------

### STRING
Strings are text values enclosed in double-quotation marks `"`. Use `\"` to escape a
double-quote and `\\` to escape a backslash.

### INTEGER
Integers are whole numbers. They can be prefixed with `-` for negative numbers.

### FLOAT
Floats are floating point numbers. They contain a single `.` in them. They can be
prefixed with `-` for negative numbers.

Costumes
--------

### Syntax
```
costumes <STRING,...>;
```

This statement is used to declare what costumes are added into a sprite. Write the path
to a image file in a String. This path is relative to the project directory. 

globals and listglobals
-----------------------

### Syntax
```
globals <NAME, ...>;
listglobals <NAME, ...>;
```

These statements are used to declare what variables and lists are `For all sprites`.

Custom Blocks (Functions)
-------------------------

### Syntax
```
def BLOCK_NAME <ARGUMENTS, ...> {
    ...
}
```

Write `nowarp` before `def` to declare a function to be `Run without screen refresh`
**unchecked**.

### Calling Functions
Function calling have the same syntax as [Statement Blocks](#statement-blocks).
```
BLOCK_NAME <EXPRESSION, ...>;
```

### Using Arguments inside Function definition
Names prefixed with a `$` sign are used to get a function's arguments.
```
say $ArgumentName;
```

When I Receive
--------------

### Syntax
```
on STRING {
    ...
}
```

Write the name of broadcast after `on`.

Macros
------

See [/docs/macros.md](/docs/macros.md)

Hat Blocks
----------

### Syntax
```
NAME <ARGUMENTS, ...> {
    ...
}
```

See [/docs/hats.md](/docs/hats.md) for a list of available Hat blocks.

### When Flag Clicked
```
onflag {
    ...
}
```

### When Space Key Pressed
```
onkey "space" {
    ...
}
```

Expressions
-----------

The usual C-like expressions are supported, including infix operators, brackets for
overriding precedence.

### Operators

| Operator | Operator Block
|----------|------
| +        | add
| -        | subtract
| *        | multiply
| /        | divide
| %        | mod
| &        | join
| and      | and
| or       | or
| not      | not

Statement Blocks
----------------

## Syntax
```
NAME <ARGUMENTS, ...>;
```

See [/docs/statements.md](/docs/statements.md) for a list of available statement blocks.

### Stop This Script Block
```
return;
```

### Ask and Wait Block
```
ask "question?";
```

Reporter Blocks
---------------

### Syntax
```
NAME(<ARGUMENTS, ...>)
```

See [/docs/reporters.md](/docs/statements.md) for a list of available statement blocks.

### [abs] of (-1) Block
```
abs(-1)
```

Variables
---------

To declare a variable and assign a value to it. 
```
NAME = <EXPRESSION>;
```

This turns into a 'set variable to' block.

Unless `NAME` is added to a [globals](#globals-and-listglobals) statement, this variable
will be `For this sprite only`.

Variables can be used by their names in expressions.

```
say NAME;
```

Lists
-----

To declare a list and delete all items of that list.
```
NAME[];
```
This turns into a 'delete all items of list' block.

### Replace item of list with
```
NAME[<INDEX>] = <EXPRESSION>;
```

### Hide/Show List monitor
```
NAME.hide;
NAME.show;
```

### Delete item at index of list
```
NAME.delete <INDEX>;
```

### Insert item at index of list
```
NAME.insert <INDEX>, <EXPRESSION>;
```

### Item # of item in list
```
NAME.index(<EXPRESSION>)
```

### List contains item
```
NAME.contains(<EXPRESSION>)
```

### Length of list
```
NAME.length
```

### Item N of list
```
NAME[<INDEX>]
```

Control-flow Statements
-----------------------

### IF
```
if <CONDITION> {
    ...
}
```

### IF-ELSE
```
if <CONDITION> {
    ...
} else {
    ...
}
```

### IF-ELSE-IF
```
if <condition> {
    ...
} elif <condition> {
    ...
}
```

### IF-ELSE-IF-ELSE
```
if <condition> {
    ...
} elif <condition> {
    ...
} else {
    ...
}
```

### Repeat Until
```
until <CONDITION> {
    ...
}
```

### Repeat Forever
```
forever {
    ...
}
```

### Repeat N times
```
repeat <EXPRESSION>  {
    ...
}
```
