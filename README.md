GoboScript
==========

GoboScript is a text-based programming language that transpiles 1:1 to Scratch.

Overview
--------

GoboScript lets you create Scratch projects from text files.

GoboScript code                            | Generated blocks
-------------------------------------------|--------------------------------------------------
![](/docs/img/example_helloworld_main.png) | ![](/docs/img/example_helloworld_main_blocks.png)

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
