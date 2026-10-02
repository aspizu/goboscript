<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/aspizu/goboscript/main/docs/assets/og-image-dark.webp" />
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/aspizu/goboscript/main/docs/assets/og-image-light.webp" />
  <img alt="goboscript screenshot" src="https://raw.githubusercontent.com/aspizu/goboscript/main/docs/assets/og-image-light.webp" />
</picture>

## Introduction

This VS Code extension adds support for the goboscript programming language, which
compiles to Scratch. It provides syntax highlighting based on the compiler's grammar,
and compile-on-save diagnostics.

## Installation

Install from the Visual Studio Code Marketplace or by searching within VS Code. See the
[installation guide](https://aspiz.uk/goboscript/docs) for installing the compiler.

## Compile on save

Saving a `.gs` file runs `goboscript build` on the enclosing project (the nearest
directory containing `stage.gs` or `goboscript.toml`) and shows the reported errors
and warnings in the **Problems** panel. The build also writes the project's `.sb3`
file. The `goboscript` compiler must be installed and available on your `PATH`,
see the [installation guide](https://aspiz.uk/goboscript/docs).

## `.sb3` preview

Opening a `.sb3` file (the output of `goboscript build`) shows it running in the
embedded [TurboWarp Scaffolding](https://github.com/TurboWarp/scaffolding) player,
fully offline. The preview shows exactly the bytes on disk when it opens; press
**Reload** to load the file again after rebuilding. It never builds or watches
anything itself. The stage is scaled to fit the panel while preserving its
aspect ratio, and the project stays paused until you press **Green flag**.

## Documentation

See the [VS Code extension guide](https://aspiz.uk/goboscript/docs/editor-integration/vscode.html)
for setup, builds on save, and the project preview and console.

Please see the [goboscript documentation](https://aspiz.uk/goboscript/docs) for using
goboscript.

## Reporting Issues

Issues should be reported in the
[goboscript issue tracker](https://github.com/aspizu/goboscript/issues).
