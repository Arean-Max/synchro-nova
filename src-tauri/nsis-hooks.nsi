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

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$SMPROGRAMS\Synchro.lnk"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro"
  DeleteRegKey HKCU "Software\Classes\Applications\synchro.exe"
!macroend
