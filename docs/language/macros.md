# Macros

goboscript has a C-like preprocessor. This allows you to define macros and
include files.

!!! note
    The preprocessor directives start with a `%` character. The `%` character must
    always appear at the start of a line. There cannot be any indentation before the
    `%` character.

## Include

Include the contents of a file.

```goboscript
%include path/to/file.gs
```

The `.gs` extension is optional. If not specified (recommended), the file extension will
be added automatically.

If the include path is a directory, the file inside the directory with the same name as
the directory will be included.

By default, the include path is relative to the project root directory. To include a
file relative to the current file, use `./` or `../`

!!! tip
    [`bkpk.py`](https://gist.github.com/aspizu/c81452bfb7a333d0819f0279e51e078a) is a small
    Python script that lets you include files from the internet using `%include` directives.

    ```goboscript
    # run `./bkpk.py` to compile your project, instead of `goboscript build`
    %include https://github.com/username/repo/branchname/filename.gs
    ```

## Define

Define a macro. That identifier will be substituted with the subsequent text.

```goboscript
%define macro_name replacement text
```

## Define with arguments

Define a macro with arguments. The arguments will be substituted with the tokens from
the callsite.

```goboscript
%define macro_name(arg1, arg2) replacement text
```

Since `()` are interpreted as function parameter brackets, use double parentheses to include them in the expansion:

```goboscript
%define foo ((1 + 2))
```

This expands to `((1 + 2))`, allowing you to control operator precedence in macro substitutions.

Use `\` at the end of a line to continue the replacement text across multiple lines:

```goboscript
%define long_macro this is a very long \
                   replacement text that spans \
                   multiple lines
```

## Define with overloaded arguments

Macros with arguments can be overloaded by defining multiple versions with different
numbers of arguments. The correct version will be selected based on the number of
arguments passed at the callsite.
```goboscript
%define MACRO(A) "MACRO(A)"
%define MACRO(A, B) "MACRO(A, B)"

onflag {
    say MACRO(1);      # expands to "MACRO(A)"
    say MACRO(1, 1);   # expands to "MACRO(A, B)"
}
```

Each overload is stored independently, so defining `MACRO` with one argument does not
affect the definition of `MACRO` with two arguments. Using `%undef macro_name` removes
all overloads for that name at once.

## Remove a macro definition

```goboscript
%undef macro_name
```

## Conditional compilation

```goboscript
%if macro_name
    code
%endif
```

```goboscript
%if not macro_name
    code
%endif
```

## Format Strings

`FMT()` is a built-in macro that builds a string by interpolating values into a format string. The first argument must be a string literal containing `%s` placeholders. Each subsequent argument is substituted in order for each `%s`.

```goboscript
FMT("Hello, %s! You are %s years old.", name, age)
```

This expands to a chain of `&` (join) operations:

```goboscript
"Hello, " & name & "! You are " & age & " years old."
```

The number of arguments after the format string must exactly match the number of `%s` placeholders in the format string. Passing too few or too many arguments is a compile-time error.

```goboscript
FMT("x = %s, y = %s", x, y)   # correct: 2 placeholders, 2 arguments
FMT("x = %s", x, y)           # error: 1 placeholder, 2 arguments given
FMT("x = %s, y = %s", x)      # error: 2 placeholders, 1 argument given
```

If no placeholders are present (only one argument), `FMT` simply expands to that string literal:

```goboscript
FMT("no placeholders")   # expands to "no placeholders"
```

## Concatenate Tokens

```goboscript
CONCAT(prefix, suffix) # becomes prefixsuffix
```
