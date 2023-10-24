#!/bin/bash
set -e

if command -v doas &> /dev/null; then
  alias sudo=doas
elif command -v sudo &> /dev/null; then
  true
else
  echo "Either sudo or doas is required."
  exit 1
fi

if command -v pip3 &> /dev/null; then
  alias pip=pip3
fi

IFS=":" read -ra path_dirs <<< "$PATH"

for f in "${path_dirs[@]}"; do
  if [[ "$f"/ = "$HOME"/* ]]; then
    scripts="$f"
    break
  fi
done

if test -z "$scripts"; then
  echo "Could not find a directory inside \$PATH which is inside your home directory."
  echo
  echo "Consider adding this to ~/.bash_profile or ~/.zshenv"
  echo "export PATH=~/.local/bin:\"$PATH\""
  exit 1
fi

if test $OSTYPE = "linux-gnu"; then
  if test -f /etc/os-release; then
    source /etc/os-release
    case $NAME in
      ("Arch Linux")
        sudo pacman -S --noconfirm --needed python python-pip python-lark-parser python-setuptools
        skip_pip=true
        ;;
      ("Ubuntu" | "Debian")
        sudo apt --yes install python3 python3-pip python3-setuptools python3-lark
        skip_pip=true
        ;;
      (*)
        ;;
    esac
  fi
elif test $OSTYPE = "darwin"; then
  echo
fi

if test -z "$skip_pip"; then
  if ! command -v pip3 &> /dev/null && ! command -v pip &> /dev/null; then
    if command -v python3 &> /dev/null || command -v python &> /dev/null; then
      echo "Python was found but pip was not found."
    else
      echo "Python is not installed."
    fi
    exit 1
  fi
  pip install lark setuptools
fi
pip install --break-system-packages --editable .
echo -e '#!/bin/sh\nexec python -m goboscript "$@"' > $scripts/gsc
chmod +x $scripts/gsc

echo
echo
echo "The goboscript compiler was installed, use the command gsc to run it."
