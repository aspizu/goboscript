# Editor Integration

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
