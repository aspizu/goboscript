# Costumes

You can add costumes to a sprite by specifying their file paths relative to the project directory.

```goboscript
costumes "path/to/costume.svg";
```

### Listing Multiple Costumes
To add multiple costumes, separate their file paths with commas. Costumes will appear in the order you list them in the `costumes` statement.

```goboscript
costumes "path/to/costume1.svg", "path/to/costume2.svg";
```

Each costume's name is taken from the file name without its extension.

### Renaming Costumes
You can rename a costume using the `as` keyword.

```goboscript
costumes "path/to/costume.svg" as "new name";
```

### Using Wildcards (Globs)
You can use wildcards to include multiple costumes, such as all `.svg` files in a directory. Use the `*` wildcard for this.

```goboscript
costumes "path/to/costumes/*.svg";
```

Costumes added this way are sorted alphabetically.

## Generating costumes for text engines and case detection

Scratch compares strings case-insensitively. Switching costumes is, however,
case-sensitive. This can be utilized to detect the case of a character by first switching
to the costume named by the character, then using the costume number to detect the case.

For example, if you have a costume named "A" at position 1, and a costume named "a" at
position 2, you can use the following code to detect the case of a character:

```goboscript
switch_costume char;
if costume_number() == 1 {
    say "upper case A";
} else {
    say "lower case a";
}
```

Its useful to have one costume for each printable character in the ASCII set. This will
allow you to get the ASCII value of any printable character.

Writing `costumes "blank.svg" as "A", "blank.svg" as "B" ...` for each printable
character is a pain.

goboscript provides a special declaration for generating such costumes automatically.

```goboscript
costumes "blank.svg" as "@ascii/PREFIX";
```

This will generate costumes for all printable characters in the ASCII set, with the
prefix "PREFIX". For example, if the prefix is "A", the costumes will be named "A0",
"A1", etc.

If you do not wish to have a prefix, leave it blank. (i.e. `@ascii/`)

Given that these are placed at the beginning of the costumes list, you can get the
ASCII value of a character by adding `31` to the costume number.
