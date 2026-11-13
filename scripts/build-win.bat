@echo off
cd /d "%~dp0\.."
cargo build --release
if errorlevel 1 (
    echo Ehitamine ebaonnestus.
) else (
    echo.
    echo Valmis: target\release\vaarismetallid.exe
)
pause
