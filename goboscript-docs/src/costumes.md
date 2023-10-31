# Costumes

Each sprite must have atleast one costumes declaration. Write paths in strings
separated by commas. Costume paths are relative to the project directory.

```goboscript
costumes "blank.svg", "player.png";
```

Globs can be used to include costumes based on a pattern.

```goboscript
costumes "frames/*.svg";
```

This will include all files ending with `.svg` from the `frames/` directory sorted
alphabetically.

## Costume names

Costumes by default will be named their file-names minus the file extension. So, the
costume `blank.svg` will be named `blank` in the sprite.

```admonish tip
As unix file-names cannot include a `/` character, if a costume file-name includes
`{{fwslash}}` then it will be replaced with a `/` character. So, the costume
`char-{{fwslash}}.png` will be named `char-/` in the sprite.
```

# Costume machines

goboscript comes with powerful tools to generate costumes programmatically.

## `*machine:ASCII`

Include `blank.svg` with names being ASCII characters from 33 to 127.

```goboscript
costumes "*machine:ASCII";
```
