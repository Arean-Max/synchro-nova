@echo off
setlocal
cd /d "%~dp0"

set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

where cargo >nul 2>nul
if errorlevel 1 (
  echo Cargo was not found. Please install Rust (https://rustup.rs) and try again.
  pause
  exit /b 1
)

where npm >nul 2>nul
if errorlevel 1 (
  echo Node.js / npm was not found. Please install Node.js (https://nodejs.org) and try again.
  pause
  exit /b 1
)

if not exist "node_modules" (
  echo Installing dependencies...
  call npm install
  if errorlevel 1 (
    echo npm install failed.
    pause
    exit /b 1
  )
)

if not exist "dist" mkdir "dist"
if exist "dist\Synchro.exe" del /f /q "dist\Synchro.exe"
if exist "dist\Synchro.pdb" del /f /q "dist\Synchro.pdb"
if exist "dist\Synchro.map" del /f /q "dist\Synchro.map"

echo Building Synchro...
call npx tauri build --no-bundle
if errorlevel 1 (
  echo Build failed.
  pause
  exit /b 1
)

copy /Y "src-tauri\target\release\synchro.exe" "dist\Synchro.exe" >nul
if exist "src-tauri\target\release\synchro.pdb" del /f /q "src-tauri\target\release\synchro.pdb"
if exist "src-tauri\target\release\synchro.d" del /f /q "src-tauri\target\release\synchro.d"

for /f "delims=" %%S in ('rustup which llvm-strip 2^>nul') do set "LLVM_STRIP=%%S"
if defined LLVM_STRIP (
  if exist "%LLVM_STRIP%" "%LLVM_STRIP%" -s "dist\Synchro.exe" >nul 2>nul
)

echo.
echo Build complete: dist\Synchro.exe
pause
