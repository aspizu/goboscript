#!/bin/bash
set -e
pip install lark
mkdir -p ~/.local/bin
cat << EOF > ~/.local/bin/gsc
#!/bin/bash
python3 $PWD/gsc \$@
EOF
chmod +x ~/.local/bin/gsc
