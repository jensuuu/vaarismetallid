#!/usr/bin/env bash
# Ehitab Windowsi .exe faili Linuxis (ristkompileerimine).
# Fedora: sudo dnf install mingw64-gcc
set -e
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
echo "Valmis: target/x86_64-pc-windows-gnu/release/vaarismetallid.exe"
