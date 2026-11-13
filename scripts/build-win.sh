#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/yhine.sh"
mine_juurkausta
kontrolli_cargo
kontrolli_mingw

echo "==> Lisan Windowsi sihtmärgi (kui puudub)..."
rustup target add x86_64-pc-windows-gnu

echo "==> Ehitan Windowsi versiooni..."
cargo build --release --target x86_64-pc-windows-gnu

mkdir -p dist/windows
cp target/x86_64-pc-windows-gnu/release/vaarismetallid.exe dist/windows/
echo
echo "Valmis: dist/windows/vaarismetallid.exe"
