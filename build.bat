@echo off
cd /d "%~dp0"
echo Installing dependencies...
call npm install
if %errorlevel% neq 0 (
    echo Install failed!
    pause
    exit /b 1
)
echo Building MSI installer...
call npm run tauri build
if %errorlevel% neq 0 (
    echo Build failed!
    pause
    exit /b 1
)
echo Build complete! Output: src-tauri\target\release\bundle\msi\
explorer "src-tauri\target\release\bundle\msi"
pause
