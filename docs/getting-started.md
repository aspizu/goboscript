# Getting Started

goboscript builds a Scratch project from a folder which holds the code, costumes and
sounds.

Each sprite is a separate file in the folder with the `.gs` extension.

For example, the following folder structure:

```
project/
  stage.gs
  sprite1.gs
  sprite2.gs
  sprite3.gs
  costume1.png
  costume2.png
  sound1.wav
  sound2.wav
```

Note that the stage file is named `stage.gs` and it must be present in every project.

To add costumes or sounds to a sprite, write the path to the file.

```goboscript
costumes "costume1.png", "costume2.png";
```

You can organize assets for sprites in sub-folders.

```goboscript
costumes "sprite1/costume1.png", "sprite1/costume2.png";
```

where the folder `sprite1` holds the assets for the sprite `sprite1.gs`.

Instead of having to write the path for every asset, you can use a glob pattern.

```goboscript
costumes "sprite1/*.png";
```

See documentation for [costumes](language/costumes.md) and
[sounds](language/sounds.md).

Follow instructions at [Install](install.md) to install goboscript, then invoke
`goboscript` from a terminal to build the project.

```sh
goboscript build
```

This will generate a `project.sb3` file inside the project folder because the name
of the folder is `project`.
