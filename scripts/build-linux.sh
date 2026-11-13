#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/yhine.sh"
mine_juurkausta
kontrolli_cargo

echo "==> Ehitan Linuxi versiooni..."
cargo build --release

mkdir -p dist/linux
cp target/release/vaarismetallid dist/linux/
echo
echo "Valmis: dist/linux/vaarismetallid"
