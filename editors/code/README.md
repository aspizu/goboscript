# Contributing

To develop the extension, open the `editors/code` directory in Visual Studio Code.

```sh
cd editors/code
npm install
```

Go to the Run and Debug view and select `Launch`. This will open a new instance of VS
Code with the extension enabled.

# Installation from Source

To install the extension from source, open the `editors/code` directory in Visual Studio
Code.

```sh
cd editors/code
npm install
npm run package
```

Then in Visual Studio Code, press `Ctrl` + `Shift` + `P` and run the
`Extensions: Install from VSIX...` command.
Select the `editors/code/goboscript.vsix` file to install the extension.
