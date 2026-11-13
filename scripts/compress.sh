#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/yhine.sh"
mine_juurkausta

bash scripts/ehita_linux.sh
bash scripts/ehita_windows.sh

echo "==> Pakin..."
tar -czf dist/vaarismetallid-linux.tar.gz -C dist/linux vaarismetallid

if command -v zip > /dev/null 2>&1; then
    (cd dist/windows && zip -q ../vaarismetallid-windows.zip vaarismetallid.exe)
else
    (cd dist/windows && python3 -m zipfile -c ../vaarismetallid-windows.zip vaarismetallid.exe)
fi

echo
echo "Valmis:"
echo "  dist/vaarismetallid-linux.tar.gz"
echo "  dist/vaarismetallid-windows.zip"
