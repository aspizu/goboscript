# Getting Started

The goboscript compiler is a command-line program. You can create a new project using
the `new` command. (run `goboscript new --help` for more information)

## Create a new project

Create a new folder, and make sure that your working-directory is set to that folder.

```shell
goboscript new
```

This will create a new project with the following structure:

```
.
├── blank.svg
├── main.gs
├── stage.gs
└── goboscript.toml
```

Depending on what options you chose, it might create a `goboscript.toml` configuration
file.

Each `.gs` file holds the code for a sprite, the name of the sprite is the name of
the file without the `.gs` extension.

`stage.gs` holds the code for the Stage. Scratch does not allow you to name a sprite
`Stage`, so creating a file with the name `Stage.gs` is invalid. As goboscript
uses `stage.gs` for the Stage, you also cannot name a sprite `stage` (in lowercase).

`blank.svg` is a blank costume. You can see that both the main sprite and the Stage have
the line:

```goboscript
costumes "blank.svg";
```

This is used to add a costume to a sprite (or the Stage), see
[language/costumes](./language/costumes.md) for more information.

## Compile the project

To compile the project, run the following command:

```shell
goboscript build
```

This will compile the project into a `.sb3` file. The `.sb3` file will be placed in the
project directory. It will have the same name as the project directory.

Run `goboscript build --help` for more information.
