#!/bin/bash

if [[ ":$PATH:" != *":$HOME:/.local/bin"* ]]; then
  echo "ERROR: ~/.local/bin is not in your PATH variable."
  echo "Installation cannot proceed."
  exit 100
fi

if ! command -v pip > /dev/null; then
  echo "ERROR: pip command not found."
  echo "Installation cannot proceed."
  exit 100
fi

if ! command -v git > /dev/null; then
  echo "ERROR: git command not found."
  echo "Installation cannot proceed."
  exit 100
fi

pip install --upgrade lark

cd ~
git clone https://github.com/aspizu/goboscript
echo -e "#\!/bin/bash\nset -e\npython $(pwd)/gsc "'"$@"' > ~/.local/bin/gsc
chmod +x ~/.local/bin/gsc
