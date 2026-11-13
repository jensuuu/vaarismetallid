#!/usr/bin/env bash

kontrolli_cargo() {
    if ! command -v cargo > /dev/null 2>&1; then
        echo "VIGA: Cargo puudub. Paigalda Rust: https://rustup.rs"
        exit 1
    fi
}

kontrolli_mingw() {
    if ! command -v x86_64-w64-mingw32-gcc > /dev/null 2>&1; then
        echo "VIGA: mingw puudub (vajalik Windowsi .exe ehitamiseks)."
        echo "  Fedora:        sudo dnf install mingw64-gcc"
        echo "  Debian/Ubuntu: sudo apt install gcc-mingw-w64-x86-64"
        exit 1
    fi
}

# Liigub projekti juurkausta, olenemata sellest, kust skript käivitati
mine_juurkausta() {
    cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.."
}
