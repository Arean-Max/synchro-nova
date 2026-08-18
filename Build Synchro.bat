@echo off
setlocal
cd /d "%~dp0"
set "CARGO_BUILD_JOBS=1"
set "CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG_ASSERTIONS=true"
set "CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true"
set "CARGO_PROFILE_RELEASE_OPT_LEVEL=0"
set "CARGO_PROFILE_RELEASE_LTO=false"
set "CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16"
set "CODEX_NODE=%USERPROFILE%\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\bin"
if not exist ".tmp" mkdir ".tmp"
set "TEMP=%~dp0.tmp"
set "TMP=%~dp0.tmp"

if exist "%CODEX_NODE%\node.exe" (
  set "PATH=%CODEX_NODE%;%PATH%"
)

set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
where cargo >nul 2>nul
if errorlevel 1 (
  echo cargo was not found. Install Rust and run this file again.
  pause
  exit /b 1
)

if not exist "node_modules\.bin\tauri.cmd" (
  if not exist "%USERPROFILE%\.cache\codex-runtimes\codex-primary-runtime\dependencies\bin\pnpm.cmd" (
    echo Node package manager was not found. Install Node.js LTS and run this file again.
    pause
    exit /b 1
  )
  call "%USERPROFILE%\.cache\codex-runtimes\codex-primary-runtime\dependencies\bin\pnpm.cmd" install
  if errorlevel 1 (
    pause
    exit /b 1
  )
)

if not exist "dist" mkdir "dist"
if exist "dist\Synchro.exe" del /f /q "dist\Synchro.exe"
if exist "dist\Synchro.pdb" del /f /q "dist\Synchro.pdb"
if exist "dist\Synchro.map" del /f /q "dist\Synchro.map"

call .\node_modules\.bin\tauri.cmd build --no-bundle
if errorlevel 1 (
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

echo Built single executable: dist\Synchro.exe
pause
