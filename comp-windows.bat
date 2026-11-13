@echo off
REM Ehitab programmi Windowsis. Vajab Rusti (https://rustup.rs)
cargo build --release
if errorlevel 1 (
    echo Ehitamine ebaonnestus.
) else (
    echo.
    echo Valmis: target\release\vaarismetallid.exe
)
pause
