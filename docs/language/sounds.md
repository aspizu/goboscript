# Sounds

!!! note
    Scratch only supports MP3 and WAV files.
    Other formats will (as of now) not raise a warning or error, and the project will
    refuse to load in Scratch.

You can add sounds to a sprite by specifying their file paths relative to the project
directory.

```goboscript
sounds "path/to/sound.mp3";
```

### Listing Multiple Sounds
To add multiple sounds, separate their file paths with commas. Sounds will appear in the
order you list them in the `sounds` statement.

```goboscript
sounds "path/to/sound1.mp3", "path/to/sound2.wav";
```

Each sound's name is taken from the file name without its extension.

### Renaming Sounds
You can rename a sound using the `as` keyword.

```goboscript
sounds "path/to/sound.mp3" as "new name";
```

### Using Wildcards (Globs)
You can use wildcards to include multiple sounds, such as all `.mp3` files in a
directory. Use the `*` wildcard for this.

```goboscript
sounds "path/to/sounds/*.mp3";
```

Sounds added this way are sorted alphabetically.
