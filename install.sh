#!/bin/bash

echo "PATH=$PATH" >> log.out
echo "OSTYPE=$OSTYPE" >> log.out
echo "--- /etc/os-release" >> log.out
cat /etc/os-release >> log.out
echo "---" >> log.out

set -e

get_bindir() {
  IFS=":" read -ra path_dirs <<< "$PATH"
  for f in "${path_dirs[@]}"; do
    if [[ "$f"/ = "$HOME"/* ]]; then
      BINDIR="$f"
      break
    fi
  done
  if test -z "$BINDIR"; then
    echo "Could not find a directory inside \$PATH which is inside your home directory."
    echo
    echo "Consider adding this to ~/.bash_profile or ~/.zshenv"
    echo "export PATH=~/.local/bin:\"\$PATH\""
    exit 1
  fi
  echo "BINDIR=$BINDIR" >> log.out
}

has_command() {
  command -v "$1" &> /dev/null
}

get_command_pip() {
  if has_command pip3.11; then
    PIP=pip3.11
  elif has_command pip311; then
    PIP=pip311
  elif has_command pip3; then
    PIP=pip3
  elif has_command pip; then
    PIP=pip
  else
    get_command_python
    if ! test -z "$PYTHON"; then
      PIP="$PYTHON -m pip"
    fi
  fi
  if ! test -z "$PIP"; then
    set +e
    $PIP install --break-system-packages &> /dev/null
    if test "$?" -eq 1; then
      PIP_INSTALL="$PIP install --break-system-packages"
    else
      PIP_INSTALL="$PIP install"
    fi
    set -e
    echo "PIP_INSTALL=$PIP" >> log.out
    echo "PIP=$PIP" >> log.out
  fi
}

get_command_python() {
  if has_command python3.11; then
    PYTHON=python3.11
  elif has_command python311; then
    PYTHON=python311
  elif has_command python3; then
    PYTHON=python3
  elif has_command python; then
    PYTHON=python
  elif has_command py3.11; then
    PYTHON=py3.11
  elif has_command py311; then
    PYTHON=py311
  elif has_command py3; then
    PYTHON=py3
  elif has_command py; then
    PYTHON=py
  fi
}

has_python_package() {
  echo "import $1" | $PYTHON &> /dev/null
}

install_package() {
  $PIP_INSTALL --editable .
}

install_command() {
  mkdir -p "$BINDIR"
  get_command_python
  echo -e '#!/bin/sh\nexec '"$PYTHON"' -m goboscript "$@"' > "$BINDIR"/gsc
  chmod +x "$BINDIR/gsc"
  echo "Goboscript is installed, Use the gsc command to run it."
}

archlinux() {
  sudo pacman -S --noconfirm --needed python python-pip python-lark-parser python-setuptools
  get_command_pip
  install_package
  get_bindir
  install_command
}

voidlinux() {
  sudo xbps-install -y python3 python3-pip python3-setuptools
  get_command_pip
  $PIP_INSTALL lark
  install_package
  get_bindir
  install_command
}

fedora() {
  sudo dnf install -y python3 python3-pip python3-setuptools python3-lark*
  get_command_pip
  install_package
  get_bindir
  install_command
}

debian() {
  sudo apt install --yes python3 python3-pip python3-setuptools python3-lark
  get_command_pip
  install_package
  get_bindir
  install_command
}

haiku() {
  pkgman install -y python3.11 pip_python311 setuptools_python311
  get_command_pip
  if test -z "$PIP"; then
    echo
    echo "Re-run this script after rebooting."
    exit
  fi
  $PIP_INSTALL lark
  install_package
  BINDIR=~/config/non-packaged/bin
  install_command
}

unknown_os() {
  get_command_pip
  if test -z "$PIP"; then
    echo "Could not find the pip command. Install python and pip using your system's"
    echo "package manager."
    exit 1
  fi
  get_command_python
  if ! has_python_package setuptools; then
    echo "Could not find the setuptools package. Install setuptools using your system's"
    echo "package manager or using pip."
    exit 1
  fi
  if ! has_python_package lark; then
    echo "Could not find the lark package. Install lark using your system's package"
    echo "package manager or using pip."
    exit 1
  fi
  install_package
  get_bindir
  install_command
}

if test -f /etc/os-release; then
  source /etc/os-release
  case "$NAME" in
    ("Arch Linux"|"CachyOS Linux")
      archlinux
      exit
      ;;
    ("Void Linux")
      voidlinux
      exit
      ;;
    ("Ubuntu" | "Debian")
      debian
      exit
      ;;
    ("Fedora" | "Red Hat"*)
      fedora
      exit
      ;;
    (*)
      ;;
  esac
fi

case "$OSTYPE" in
  ("haiku")
    haiku
    exit
    ;;
  (*)
    ;;
esac

unknown_os
exit
