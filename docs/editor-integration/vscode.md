## Visual Studio Code

The VSCode extension provides code snippets, syntax highlighting and diagnostics.

### Install from VSCode Marketplace

Search goboscript in the Extensions tab, or go to the [VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=aspizu.goboscript) to install it.

### Install from source

You will need `npm` installed.

```bash
# Inside the root of the goboscript git repository:
cd editors/code
npm install
npm run package
```

This will output a `goboscript-x.y.z.vsix` file in the `editors/code` directory. You can
install it by pressing ++ctrl+shift+p++ and typing `Extensions: Install from VSIX...`.

### Configure Build Task

You can configure the goboscript build task to get diagnostics in vscode.

Add to `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "goboscript-build",
      "problemMatcher": ["$rustc"],
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "presentation": {
        "clear": true,
        "reveal": "never"
      },
      "label": "Build .sb3"
    }
  ]
}
```

Either press ++ctrl+shift+b++ or run `Tasks: Run Task` and select `Build .sb3`.
