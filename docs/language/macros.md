# Macros

goboscript has a C-like preprocessor. This allows you to define macros and
include files.

## Include

Include the contents of a file.

```goboscript
%include path/to/file.gs
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
