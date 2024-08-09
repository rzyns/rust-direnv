#!/usr/bin/env bash
sudo BINDIR=/usr/local/bin bash -ic "$(curl -fsLS get.chezmoi.io)"
chezmoi init --apply "rzyns"
mise install -y
