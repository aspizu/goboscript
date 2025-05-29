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

You can use a backslash `\` at the end of a line to continue the replacement text onto the next line:

```goboscript
%define long_macro this is a very long \
                   replacement text that spans \
                   multiple lines
```

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

## Concatenate Tokens

```goboscript
CONCAT(prefix, suffix) # becomes prefixsuffix
```
