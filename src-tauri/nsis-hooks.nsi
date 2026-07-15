!macro NSIS_HOOK_POSTINSTALL
  CreateShortcut "$SMPROGRAMS\Synchro.lnk" "$INSTDIR\synchro.exe" "" "$INSTDIR\synchro.exe" 0 "" "" "Synchro Nova"
  !insertmacro SetLnkAppUserModelId "$SMPROGRAMS\Synchro.lnk"

  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe" "" "$INSTDIR\synchro.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe" "Path" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro" "" "$INSTDIR\synchro.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro" "Path" "$INSTDIR"

  WriteRegStr HKCU "Software\Classes\Applications\synchro.exe" "FriendlyAppName" "Synchro"
  WriteRegStr HKCU "Software\Classes\Applications\synchro.exe" "ApplicationName" "Synchro"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ExecWait 'taskkill /F /IM synchro.exe /T'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$SMPROGRAMS\Synchro.lnk"
  Delete "$SMPROGRAMS\Synchro Nova.lnk"
  Delete "$SMPROGRAMS\Synchro Nova\Synchro Nova.lnk"
  Delete "$SMPROGRAMS\Synchro Nova\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Synchro Nova"
  Delete "$DESKTOP\Synchro.lnk"
  Delete "$DESKTOP\Synchro Nova.lnk"

  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro"
  DeleteRegKey HKCU "Software\Classes\Applications\synchro.exe"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\App Paths\synchro"
  DeleteRegKey HKLM "Software\Classes\Applications\synchro.exe"

  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro"
  DeleteRegValue HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro Nova"
  DeleteRegValue HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "Synchro Nova"

  DeleteRegKey HKCU "Software\Synchro"
  DeleteRegKey HKLM "Software\Synchro"
  DeleteRegKey HKCU "Software\SynchroNova"
  DeleteRegKey HKLM "Software\SynchroNova"
  DeleteRegKey HKCU "Software\synchro"

  RMDir /r "$LOCALAPPDATA\SynchroNova"
  RMDir /r "$LOCALAPPDATA\synchro"
  RMDir /r "$LOCALAPPDATA\app.synchro.performance"
  RMDir /r "$APPDATA\SynchroNova"
  RMDir /r "$APPDATA\synchro"
  RMDir /r "$APPDATA\app.synchro.performance"

  RMDir /r "$TEMP\SynchroNova"
  RMDir /r "$TEMP\synchro"

  RMDir /r "$INSTDIR"
!macroend
