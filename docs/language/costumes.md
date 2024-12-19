# Costumes

Costumes can be added to a sprite by referring to its path relative to the project
directory.

```goboscript
costumes "path/to/costume.svg";
```

You can list multiple costumes by separating them with a comma.

```goboscript
costumes "path/to/costume1.svg", "path/to/costume2.svg";
```

It supports globs, so you can include all `.svg` files in a directory by using the `*` wildcard.

```goboscript
costumes "path/to/costumes/*.svg";
```

The name of the costume will be the name of the file without the extension.

You can change the name of the costume by using the `as` keyword.

```goboscript
costumes "path/to/costume.svg" as "new name";
```

Costumes are ordered in the order they are listed in the `costumes` statement.

Costumes included in globs are sorted alphabetically.
