- [Automatic Installation](#automatic-installation)
- [Manual Installation](#manual-installation)

# Automatic Installation

goboscript comes with a bash script which tries to automatically install all
dependencies and install goboscript without user intervention.

Open a terminal window and type these commands.

Open some directory where you would like to store goboscript.

```sh
cd $HOME/downloads/packages
```

Clone the goboscript repository.

```sh
git clone https://github.com/aspizu/goboscript
```

Open the repository directory.

```sh
cd goboscript
```

Run the automatic install script using `bash`.

```sh
bash ./install.sh
```

If everything goes well, goboscript will be installed.

Verify that goboscript is installed and is working.

```sh
gsc
```

# Manual Installation

If the automatic install fails, continue with these instructions.

goboscript depends on the following dependencies:

- [Python](https://www.python.org/)
- [pip](https://pip.pypa.io/en/stable/)
- [setuptools](https://setuptools.pypa.io/en/latest/)
- [Lark parser](https://github.com/lark-parser/lark/)

Try to install these dependencies using your operating system's built-in package
manager. If any python module is not available, try to install it using pip.

After installing all the dependencies, re-run the automatic install script.
