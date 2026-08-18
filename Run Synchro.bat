@echo off
setlocal
cd /d "%~dp0"

if exist "dist\Synchro-fix.exe" (
  copy /Y "%~dp0dist\Synchro-fix.exe" "%~dp0dist\Synchro.exe" >nul 2>nul
  if not errorlevel 1 (
    del /f /q "%~dp0dist\Synchro-fix.exe" >nul 2>nul
    start "" "%~dp0dist\Synchro.exe"
    exit /b 0
  )
  start "" "%~dp0dist\Synchro-fix.exe"
  exit /b 0
)

if exist "dist\Synchro-next.exe" (
  copy /Y "%~dp0dist\Synchro-next.exe" "%~dp0dist\Synchro.exe" >nul 2>nul
  if not errorlevel 1 (
    del /f /q "%~dp0dist\Synchro-next.exe" >nul 2>nul
    start "" "%~dp0dist\Synchro.exe"
    exit /b 0
  )
  start "" "%~dp0dist\Synchro-next.exe"
  exit /b 0
)

if exist "dist\Synchro-new.exe" (
  copy /Y "%~dp0dist\Synchro-new.exe" "%~dp0dist\Synchro.exe" >nul 2>nul
  if not errorlevel 1 (
    del /f /q "%~dp0dist\Synchro-new.exe" >nul 2>nul
    start "" "%~dp0dist\Synchro.exe"
    exit /b 0
  )
  start "" "%~dp0dist\Synchro-new.exe"
  exit /b 0
)

if exist "dist\Synchro.exe" (
  start "" "%~dp0dist\Synchro.exe"
  exit /b 0
)

if exist "src-tauri\target\release\synchro.exe" (
  start "" "%~dp0src-tauri\target\release\synchro.exe"
  exit /b 0
)

echo Synchro.exe was not found. Run "Build Synchro.bat" first.
pause
exit /b 1
