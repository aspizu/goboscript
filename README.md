<div style="display: flex; flex-direction: column; align-items: center;
            border: 1px solid rgba(255,255,255,0.1); padding: 2em; border-radius: 0.5em;
            margin: 1em">
  <h1>Goboscript</h1>
  <p>Goboscript is a text-based programming language that transpiles 1:1 to Scratch.</p>
</div>

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
