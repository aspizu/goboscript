# VS Code

Install the [goboscript extension](https://marketplace.visualstudio.com/items?itemName=aspizu.goboscript)
for syntax highlighting, builds on save, and a Scratch project preview.

The [goboscript compiler](../install.md) must be installed and available on your
`PATH` for builds on save.

## Build on save

Saving a `.gs` file builds the project, writes its `.sb3` file, and shows compiler
errors and warnings in the **Problems** panel.

## Project preview

Open a `.sb3` file to preview it in VS Code. The preview runs offline using
[TurboWarp](https://turbowarp.org/).

Press **Start** to run the project. After rebuilding, press **Reload** to load the
updated `.sb3` file. The preview does not reload automatically.

## Console

The preview's **Console** shows messages from
[`log`, `warn`, and `error`](../language/blocks/debugger.md), along with the sprite
or clone that sent them. Messages can be filtered by level.
