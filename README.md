GoboScript
==========

GoboScript is a text-based programming language that transpiles 1:1 to Scratch.

Join our [Discord](https://discord.gg/URTsYdZe5m) server for support and discussion for GoboScript.

Overview
--------

GoboScript lets you create Scratch projects from text files.

GoboScript code                            | Generated blocks
-------------------------------------------|--------------------------------------------
![](/docs/img/example_helloworld_main.png) | ![](/docs/img/example_helloworld_main_blocks.png)

Use GoboScript's sister project: [sb2gs](https://github.com/aspizu/sb2gs) to convert your existing Scratch projects into GoboScript projects,
so you can continue working on it. sb2gs still requires some manual editing. ( See the project homepage for more information. ) 

Whats New
---------

Image lists are an easy way to load image files as lists.

```
imagelist listName "imageFilePath.png";
```

this will dump the image file as bytes. If you don't want the Alpha channel, then remove
it from the image file.

Documentation
-------------

See [DOCS.md](/docs/DOCS.md)


Installation
------------

**Dependencies**
 - [Python](https://www.python.org)
 - [Lark](https://github.com/lark-parser/lark)


**Steps**
1. Install Python
2. Use [pip](https://github.com/pypa/pip) to install Lark
3. Clone this repository
4. Add a alias or create a shell script to run gsc with Python

**Commands**
 - `cd ~/Downloads/SRC`
 - `pip install lark`
 - `git clone https://github.com/aspizu/goboscript`
 - `cd goboscript`
 - `echo -e "#\!/bin/bash\nset -e\npython $(pwd)/gsc \"\$@\"" > ~/.local/bin/gsc`
 - `chmod +x ~/.local/bin/gsc`

Contributing
------------

Pull requests are appreciated.

Before creating a pull request make sure to run black and pyright.

```sh
# in repository root
pyright . # this should not fail!
black .
```
