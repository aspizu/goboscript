# Editor Integration

## Visual Studio Code

### Install from source-code

Currently, the only way to install the Visual Studio Code extension is to build it from source-code.

You will need npm to build the extension.

```shell
cd editors/code
npm install
npm run package
```

This will output a `goboscript.vsix` file in the `editors/code` directory. You can
install it by pressing ++ctrl+shift+p++ and typing `Extensions: Install from VSIX...`.
