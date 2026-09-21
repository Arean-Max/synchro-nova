!macro NSIS_HOOK_POSTINSTALL
  ; Create Synchro.lnk shortcut in Start Menu so typing "synchro" in Windows Search matches directly
  CreateShortcut "$SMPROGRAMS\Synchro.lnk" "$INSTDIR\synchro.exe" "" "$INSTDIR\synchro.exe" 0 "" "" "Synchro Nova"
  !insertmacro SetLnkAppUserModelId "$SMPROGRAMS\Synchro.lnk"

  ; Register in App Paths for Windows Search and Run dialog
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe" "" "$INSTDIR\synchro.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe" "Path" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro" "" "$INSTDIR\synchro.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro" "Path" "$INSTDIR"

  ; Register in Applications for Windows Search indexer
  WriteRegStr HKCU "Software\Classes\Applications\synchro.exe" "FriendlyAppName" "Synchro"
  WriteRegStr HKCU "Software\Classes\Applications\synchro.exe" "ApplicationName" "Synchro"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Terminate running synchro processes before deleting files
  ExecWait 'taskkill /F /IM synchro.exe /T'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; 1. Delete all shortcuts from Start Menu and Desktop
  Delete "$SMPROGRAMS\Synchro.lnk"
  Delete "$SMPROGRAMS\Synchro Nova.lnk"
  Delete "$SMPROGRAMS\Synchro Nova\Synchro Nova.lnk"
  Delete "$SMPROGRAMS\Synchro Nova\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Synchro Nova"
  Delete "$DESKTOP\Synchro.lnk"
  Delete "$DESKTOP\Synchro Nova.lnk"

  ; 2. Wipe App Paths and Applications from registry
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro"
  DeleteRegKey HKCU "Software\Classes\Applications\synchro.exe"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro"
  DeleteRegKey HKLM "Software\Classes\Applications\synchro.exe"

  ; 3. Delete Autostart Run entries
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro"
  DeleteRegValue HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro Nova"
  DeleteRegValue HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro Nova"

  ; 4. Delete vendor Software registry keys
  DeleteRegKey HKCU "Software\Synchro"
  DeleteRegKey HKLM "Software\Synchro"
  DeleteRegKey HKCU "Software\SynchroNova"
  DeleteRegKey HKLM "Software\SynchroNova"
  DeleteRegKey HKCU "Software\synchro"

  ; 5. Wipe all AppData, LocalAppData, and WebView2 cache directories completely
  RMDir /r "$LOCALAPPDATA\SynchroNova"
  RMDir /r "$LOCALAPPDATA\synchro"
  RMDir /r "$LOCALAPPDATA\app.synchro.performance"
  RMDir /r "$APPDATA\SynchroNova"
  RMDir /r "$APPDATA\synchro"
  RMDir /r "$APPDATA\app.synchro.performance"

  ; 6. Wipe temp directories
  RMDir /r "$TEMP\SynchroNova"
  RMDir /r "$TEMP\synchro"

  ; 7. Completely wipe the entire installation folder and any leftovers
  RMDir /r "$INSTDIR"
!macroend
