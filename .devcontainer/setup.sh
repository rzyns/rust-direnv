#!/usr/bin/env bash
curl https://mise.run | sh
echo "eval \"\$(/home/codespace/.local/bin/mise activate bash)\"" >> ~/.bashrc
mise install -y
