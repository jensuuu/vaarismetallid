#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/yhine.sh"
mine_juurkausta
kontrolli_cargo

cargo build --release
install -Dm755 target/release/vaarismetallid "$HOME/.local/bin/vaarismetallid"

echo
echo "Paigaldatud: $HOME/.local/bin/vaarismetallid"
case ":$PATH:" in
    *":$HOME/.local/bin:"*) echo "Käivita käsuga: vaarismetallid" ;;
    *) echo "NB! $HOME/.local/bin pole PATH-is. Lisa see oma shelli seadetesse." ;;
esac
