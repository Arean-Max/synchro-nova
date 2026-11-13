# Synchro Nova: referencia de ajustes y modificaciones del sistema

Este documento proporciona un desglose completo y transparente de cada configuración del sistema, clave de registro y comando ejecutado por Synchro Nova. Todas las acciones son estrictamente locales, deterministas y respaldadas antes de su ejecución.

---

## Garantía de seguridad y reversión

1. **Copia de seguridad automatizada previa a la ejecución**:
Antes de modificar cualquier clave de registro, Synchro Nova crea un punto de restauración del sistema y guarda el estado actual de todas las claves de destino en un archivo de copia de seguridad `.reg` en `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Revertir con 1 clic**:
Al hacer clic en **"Откатить изменения"** (Revertir) en la aplicación, se restaura la instantánea `.reg` anterior a través de Windows `reg.exe` y se vuelven a leer los valores actuales.
3. **Ejecución de comandos reforzada**:
Todas las herramientas administrativas (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) se resuelven estrictamente desde `%SystemRoot%\System32` (evitando el secuestro elevado de PATH) y se ejecutan con tiempos de espera de vigilancia sin bloqueo (5 segundos para utilidades, 25 segundos para restauración del sistema).
4. **Lista de bloqueo difícil y seguridad antitrampas**:
Las operaciones inseguras (deshabilitar la integridad de la memoria/HVCI, parchear los temporizadores del kernel, deshabilitar el archivo de paginación o aplicar anulaciones arbitrarias de controladores) se bloquean permanentemente en el motor. La inyección de proceso cero o la conexión de memoria garantizan el 100% de cumplimiento con Easy Anti-Cheat, BattlEye, Vanguard y VAC.

---

## Desglose completo de ajustes

### 1. Ajustes a nivel de usuario (usuario estándar: no se requiere administrador)

Estos ajustes solo modifican la colmena `HKEY_CURRENT_USER`. No requieren privilegios elevados y afectan únicamente al perfil de usuario activo.

| Tweak ID | Hive & Path | Value / Command | Objetivo |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Evita la ventana emergente de la barra de juegos de Xbox al iniciar el juego |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Desactiva la aceleración del mouse de Windows (entrada lineal sin formato) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Deshabilita el identificador de publicidad de Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Desactiva los consejos personalizados basados ​​en datos de diagnóstico |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Mantiene el portapapeles estrictamente en la RAM local, desactiva la sincronización en la nube |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Desactiva la transparencia de la ventana DWM para ahorrar memoria de la GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Configura el Explorador de Windows para obtener el mejor rendimiento visual |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Elimina el retraso artificial en el inicio del escritorio de Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reduce el retraso de la animación del menú de la interfaz de usuario a 10 ms |

---

### 2. Ajustes a nivel del sistema (requiere privilegios de administrador)

Estos ajustes configuran políticas de todo el sistema (`HKEY_LOCAL_MACHINE`) o llaman a herramientas administrativas estándar de Windows (`powercfg`, `netsh`, `fsutil`). Synchro verifica la elevación antes de ejecutarlos.

#### A. Programación de juegos y multimedia (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Objetivo |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Desactiva la captura de video del DVR del juego en segundo plano para liberar el codificador GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Deshabilita el mecanismo de limitación de red de Windows durante tareas de alta prioridad |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserva el 90% del tiempo de CPU para juegos en primer plano (el valor predeterminado es 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Eleva la prioridad de las tareas del programador multimedia para juegos activos |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Permite la programación de GPU acelerada por hardware (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Desactiva las superposiciones multiplano (resuelve problemas de pantalla negra o tartamudeo en algunas GPU) |

#### B. Optimización de la pila de red (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Objetivo |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Borra la caché de resolución de DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Restaura el escalado de ventana TCP estándar RFC 1323 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Permite el escalado del lado de recepción para distribuir el procesamiento de paquetes de red entre los núcleos de la CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Desactiva la fusión de segmentos de recepción en las NIC para reducir la latencia en juegos en tiempo real |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Desactiva la notificación de congestión explícita para evitar la caída de paquetes en enrutadores más antiguos |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Borra la tabla del protocolo de resolución de direcciones local |
| `winsock-reset` | `netsh winsock reset` | Restaura el catálogo de sockets de red de Windows a un estado limpio |

#### C. Gestión de energía y almacenamiento (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Objetivo |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Activa el esquema de energía estándar de alto rendimiento de Windows |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Desbloquea y activa la combinación de energía de Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Desactiva la hibernación y elimina `hiberfil.sys`, liberando espacio en disco |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Deja de actualizar las marcas de tiempo del último acceso NTFS en cada archivo leído |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Garantiza que las notificaciones TRIM estén activas para el estado del SSD y la nivelación del desgaste. |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Crea un punto de restauración del sistema de Windows completo antes de los cambios. |

#### D. Telemetría del sistema y sincronización en segundo plano
| Tweak ID | Hive & Path | Value | Objetivo |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Deshabilita las cargas de la línea de tiempo de Windows y del historial de actividad |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Restringe la optimización de entrega de actualizaciones de Windows solo a la LAN local (sin cargas P2P a Internet) |

---

## Ajustes inseguros incluidos en la lista bloqueada

Synchro bloquea intencionalmente varios "ajustes agresivos" comunes que se encuentran en scripts de terceros que comprometen la integridad del sistema o la compatibilidad anti-trampas:

- **Desactivación de la integridad de la memoria (HVCI)/seguridad basada en virtualización**: Rechazado.
- **Deshabilitar completamente Windows Defender/Firewall**: Rechazado.
- **Deshabilitar el archivo de paginación (`pagefile.sys`)**: Rechazado (provoca fallas del juego debido a picos de memoria).
- **Desactivación de la manipulación dinámica de Tick/TSC invariante**: Rechazado (puede desincronizar los relojes y activar la heurística anti-trampas).
- **Hacking masivo de la utilidad MSI (interrupciones señalizadas de mensajes)**: Rechazado (riesgo de bloqueo de GPU/pantallas azules).
