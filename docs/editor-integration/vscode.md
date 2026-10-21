# VS Code

Install the [goboscript extension](https://marketplace.visualstudio.com/items?itemName=aspizu.goboscript)
for syntax highlighting, builds on save, and a Scratch project preview.

The [goboscript compiler](../install.md) must be installed and available on your
`PATH` for builds on save.

## Build on save

Saving a `.gs` file builds the project, writes its `.sb3` file, and shows compiler
errors and warnings in the **Problems** panel.

By default the extension runs `goboscript` from your `PATH`. Set the
`goboscript.compilerPath` setting to an absolute path to use a specific
executable.

## Project preview

Open a `.sb3` file to preview it in VS Code. The preview runs offline using
[TurboWarp](https://turbowarp.org/).

Press **Start** to run the project. After rebuilding, press **Reload** to load the
updated `.sb3` file. The preview does not reload automatically.

## Console

The preview's **Console** shows messages from
[`log`, `warn`, and `error`](../language/blocks/debugger.md), along with the sprite
or clone that sent them. Messages can be filtered by level.

## Install on nix

To install with nix, add the goboscript flake to your flake inputs.
You can then use `inputs.goboscript.packages.${system}.goboscript-vscode`
(`${system}` is a string like `x86_64-linux` or `aarch64-darwin`) to access the
packaged VSCode extension. You can then use this package much like any other
VSCode package which you would find in nixpkgs: e.g. for use with `home-manager`:

```nix
programs.vscode = {
  enable = true;
  profiles.default = {
    extensions = [
      inputs.goboscript.packages.${pkgs.stdenv.hostPlatform.system}.goboscript-vscode
    ];
  };
};
```
