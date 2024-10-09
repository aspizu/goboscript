## Visual Studio Code

The VSCode extension provides code snippets and syntax highlighting.

### Install from source

You will need `npm` installed.

```shell
# Inside the root of the goboscript git repository:
cd editors/code
npm install
npm run package
```

This will output a `goboscript.vsix` file in the `editors/code` directory. You can
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
      "problemMatcher": ["$goboscript"],
      "group": {
        "kind": "build",
        "isDefault": true
      },
      "label": "Build .sb3"
    }
  ]
}
```

Either press ++ctrl+shift+b++ or run `Tasks: Run Task` and select `Build .sb3`.
