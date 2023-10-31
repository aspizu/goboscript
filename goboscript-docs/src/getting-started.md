# Creating a goboscript project

To create a blank goboscript project, create a directory to hold project files and
open it inside a terminal window.

```sh
gsc --init
```

This will create the following files.

```
my-project/
|- stage.gobo
|- main.gobo
|- blank.svg
```

# Compiling the project

Being inside the root of the project directory, this will compile the project and
output a Scratch project with the same name as the directory.

```sh
gsc
```

You can specify a different name for the output file.

```sh
gsc -o build.sb3
```

# goboscript project structure

Each sprite gets its own `.gobo` file. You are free to arrange costumes however you
want.

The stage goes in `stage.gobo`, as such you cannot use `stage.gobo` for a sprite named
"stage".
