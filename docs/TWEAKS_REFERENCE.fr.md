# Synchro Nova — Référence des modifications et ajustements du système

Ce document fournit une ventilation complète et transparente de chaque paramètre système, clé de registre et commande exécutées par Synchro Nova. Toutes les actions sont strictement locales, déterministes et sauvegardées avant exécution.

---

## Garantie de sécurité et de retour en arrière

1. **Sauvegarde automatisée avant exécution**:
Avant de modifier une clé de registre, Synchro Nova crée un point de restauration système et enregistre l'état actuel de toutes les clés cibles dans un fichier de sauvegarde `.reg` dans `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Restauration en 1 clic**:
Cliquer sur **"Откатить изменения"** (Rollback) dans l'application restaure l'instantané `.reg` précédent via Windows `reg.exe` et relit les valeurs actuelles.
3. **Exécution renforcée des commandes**:
Tous les outils d'administration (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) sont strictement résolus à partir de `%SystemRoot%\System32` (empêchant le piratage de PATH élevé) et fonctionnent avec des délais d'attente de surveillance non bloquants (5 s pour les utilitaires, 25 s pour la restauration du système).
4. **Liste de blocage rigide et coffre-fort anti-triche**:
Les opérations dangereuses (désactivation de l'intégrité de la mémoire/HVCI, correction des minuteries du noyau, désactivation du fichier d'échange ou application de remplacements de pilotes arbitraires) sont bloquées de manière permanente dans le moteur. L’absence d’injection de processus ou d’accrochage de mémoire garantit une conformité à 100 % avec Easy Anti-Cheat, BattlEye, Vanguard et VAC.

---

## Répartition complète des réglages

### 1. Ajustements au niveau de l'utilisateur (utilisateur standard — aucun administrateur requis)

Ces ajustements modifient uniquement la ruche `HKEY_CURRENT_USER`. Ils ne nécessitent pas de privilèges élevés et affectent uniquement le profil utilisateur actif.

| Tweak ID | Hive & Path | Value / Command | But |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Empêche l'apparition d'une fenêtre contextuelle Xbox Game Bar au lancement du jeu |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Désactive l'accélération de la souris Windows (entrée linéaire brute) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Désactive l'identifiant publicitaire Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Désactive les conseils personnalisés basés sur les données de diagnostic |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Conserve le presse-papiers strictement dans la RAM locale, désactive la synchronisation cloud |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Désactive la transparence de la fenêtre DWM pour économiser la mémoire GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Configure l'Explorateur Windows pour de meilleures performances visuelles |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Supprime le délai artificiel de démarrage du bureau Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Réduit le délai d'animation du menu de l'interface utilisateur à 10 ms |

---

### 2. Ajustements au niveau du système (nécessite des privilèges d'administrateur)

Ces ajustements configurent des politiques à l'échelle du système (`HKEY_LOCAL_MACHINE`) ou appellent des outils d'administration Windows standard (`powercfg`, `netsh`, `fsutil`). Synchro vérifie l'élévation avant de les exécuter.

#### A. Planification des jeux et du multimédia (MMCSS)
| Tweak ID | Hive & Path | Value / Command | But |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Désactive la capture vidéo DVR du jeu en arrière-plan pour libérer l'encodeur GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Désactive le mécanisme de limitation du réseau Windows pendant les tâches hautement prioritaires |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Réserve 90 % du temps CPU pour les jeux de premier plan (80 % par défaut) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Élève la priorité des tâches du planificateur multimédia pour les jeux actifs |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Permet la planification GPU accélérée par le matériel (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Désactive les superpositions multi-plans (résout les problèmes d'écran noir / bégaiement sur certains GPU) |

#### B. Optimisation de la pile réseau (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | But |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Efface le cache du résolveur DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Restaure la mise à l'échelle de la fenêtre TCP standard RFC 1323 |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Permet la mise à l'échelle côté réception pour distribuer le traitement des paquets réseau sur les cœurs de processeur |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Désactive la fusion des segments de réception sur les cartes réseau pour réduire la latence dans les jeux en temps réel |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Désactive la notification explicite de congestion pour empêcher la perte de paquets sur les anciens routeurs |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Efface la table du protocole de résolution d'adresse local |
| `winsock-reset` | `netsh winsock reset` | Restaure le catalogue de sockets réseau Windows à l'état propre |

#### C. Gestion de l'alimentation et du stockage (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | But |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Active le schéma d'alimentation standard Windows High Performance |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Déverrouille et active le schéma d'alimentation Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Désactive l'hibernation et supprime « hiberfil.sys », libérant ainsi de l'espace disque |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Arrête de mettre à jour les horodatages du dernier accès NTFS à chaque lecture de fichier |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Garantit que les notifications TRIM sont actives pour la santé du SSD et le niveau d'usure |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Crée un point de restauration complet du système Windows avant les modifications |

#### D. Télémétrie du système et synchronisation en arrière-plan
| Tweak ID | Hive & Path | Value | But |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Désactive les téléchargements de la chronologie Windows et de l'historique des activités |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Restreint l'optimisation de la livraison de Windows Update au réseau local uniquement (pas de téléchargements P2P sur Internet) |

---

## Ajustements dangereux sur liste bloquée

Synchro bloque intentionnellement plusieurs « ajustements agressifs » courants trouvés dans les scripts tiers qui compromettent l'intégrité du système ou la compatibilité anti-triche :

- **Désactivation de l'intégrité de la mémoire (HVCI)/sécurité basée sur la virtualisation** : rejeté.
- **Désactivation complète de Windows Defender/Pare-feu** : Rejeté.
- **Désactivation du fichier d'échange (`pagefile.sys`)** : Rejeté (provoque des plantages du jeu en cas de pics de mémoire).
- **Désactivation de la falsification Dynamic Tick / Invariant TSC** : Rejeté (peut désynchroniser les horloges et déclencher l'heuristique anti-triche).
- **Piratage de l'utilitaire MSI (Message Signaled Interrupts) en masse** : Rejeté (risque de blocage du GPU / écrans bleus).
