# Synchro Nova — Referência de modificações e ajustes do sistema

Este documento fornece uma análise completa e transparente de cada configuração do sistema, chave de registro e comando executado pelo Synchro Nova. Todas as ações são estritamente locais, determinísticas e têm backup antes da execução.

---

## Garantia de segurança e reversão

1. **Backup automatizado de pré-execução**:
Antes de modificar qualquer chave de registro, o Synchro Nova cria um ponto de restauração do sistema e salva o estado atual de todas as chaves de destino em um arquivo de backup `.reg` em `%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg`.
2. **Reversão em 1 clique**:
Clicar em **"Откатить изменения"** (Reversão) no aplicativo restaura o instantâneo `.reg` anterior por meio do `reg.exe` do Windows e relê os valores atuais.
3. **Execução de comando reforçada**:
Todas as ferramentas administrativas (`netsh`, `powercfg`, `sc`, `bcdedit`, `reg`, `powershell`) são estritamente resolvidas em `%SystemRoot%\System32` (evitando sequestro de PATH elevado) e executadas com tempos limite de watchdog sem bloqueio (5s para utilitários, 25s para restauração do sistema).
4. **Lista de bloqueio difícil e seguro anti-cheat**:
Operações inseguras (desativar a integridade da memória/HVCI, corrigir temporizadores do kernel, desativar o arquivo de paginação ou aplicar substituições arbitrárias de driver) são permanentemente bloqueadas no mecanismo. A injeção zero de processo ou a conexão de memória garantem 100% de conformidade com Easy Anti-Cheat, BattlEye, Vanguard e VAC.

---

## Análise completa dos ajustes

### 1. Ajustes no nível do usuário (usuário padrão - sem necessidade de administrador)

Esses ajustes modificam apenas a seção `HKEY_CURRENT_USER`. Eles não exigem privilégios elevados e afetam apenas o perfil do usuário ativo.

| Tweak ID | Hive & Path | Value / Command | Propósito |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Impede o pop-up da Barra de Jogo Xbox no lançamento do jogo |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Desativa a aceleração do mouse do Windows (entrada linear bruta) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Desativa o identificador de publicidade do Windows |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Desativa dicas personalizadas baseadas em dados de diagnóstico |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Mantém a área de transferência estritamente na RAM local, desativa a sincronização na nuvem |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | Desativa a transparência da janela DWM para economizar memória GPU |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | Configura o Windows Explorer para melhor desempenho visual |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Remove atraso artificial na inicialização da área de trabalho do Windows |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Reduz o atraso da animação do menu da IU para 10 ms |

---

### 2. Ajustes no nível do sistema (requer privilégios de administrador)

Esses ajustes configuram políticas de todo o sistema (`HKEY_LOCAL_MACHINE`) ou chamam ferramentas administrativas padrão do Windows (`powercfg`, `netsh`, `fsutil`). Synchro verifica a elevação antes de executá-los.

#### A. Agendamento de jogos e multimídia (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Propósito |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Desativa a captura de vídeo DVR do jogo em segundo plano para liberar o codificador GPU |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Desativa o mecanismo de otimização de rede do Windows durante tarefas de alta prioridade |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | Reserva 90% do tempo de CPU para jogos em primeiro plano (o padrão é 80%) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Eleva a prioridade das tarefas do agendador multimídia para jogos ativos |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Permite agendamento de GPU acelerado por hardware (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Desativa sobreposições de vários planos (resolve problemas de tela preta/interrupção em algumas GPUs) |

#### B. Otimização da pilha de rede (`netsh`, `ipconfig`)
| Tweak ID | Tool / Command | Propósito |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | Limpa o cache do resolvedor DNS |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Restaura o dimensionamento de janela TCP RFC 1323 padrão |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Permite que o Receive-Side Scaling distribua o processamento de pacotes de rede entre os núcleos da CPU |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Desativa a coalescência de segmento de recebimento em NICs para reduzir a latência em jogos em tempo real |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Desativa a notificação explícita de congestionamento para evitar a queda de pacotes em roteadores mais antigos |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Limpa a tabela local do protocolo de resolução de endereços |
| `winsock-reset` | `netsh winsock reset` | Restaura o catálogo de soquetes de rede do Windows para o estado limpo |

#### C. Gerenciamento de energia e armazenamento (`powercfg`, `fsutil`)
| Tweak ID | Tool / Command | Propósito |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Ativa o esquema de energia padrão do Windows de alto desempenho |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Desbloqueia e ativa o esquema de energia do Windows Ultimate Performance |
| `hibernate-off` | `powercfg /hibernate off` | Desativa a hibernação e exclui `hiberfil.sys`, liberando espaço em disco |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Pára de atualizar os carimbos de data e hora do último acesso NTFS em cada leitura de arquivo |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | Garante que as notificações TRIM estejam ativas para integridade do SSD e nivelamento de desgaste |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Cria um ponto completo de restauração do sistema Windows antes das alterações |

#### D. Telemetria do sistema e sincronização em segundo plano
| Tweak ID | Hive & Path | Value | Propósito |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Desativa a linha do tempo do Windows e os uploads do histórico de atividades |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Restringe a otimização de entrega do Windows Update apenas à LAN local (sem uploads P2P para a Internet) |

---

## Ajustes inseguros na lista de bloqueio

O Synchro bloqueia intencionalmente vários “ajustes agressivos” comuns encontrados em scripts de terceiros que comprometem a integridade do sistema ou a compatibilidade anti-cheat:

- **Desativação da integridade da memória (HVCI)/segurança baseada em virtualização**: rejeitado.
- **Desativando totalmente o Windows Defender/Firewall**: Rejeitado.
- **Desativando arquivo de paginação (`pagefile.sys`)**: Rejeitado (causa travamentos do jogo em picos de memória).
- **Desativando Tick Dinâmico / Adulteração Invariável de TSC**: Rejeitado (pode dessincronizar relógios e desarmar heurísticas anti-cheat).
- **Hacking do utilitário MSI (Message Signaled Interrupts) em massa**: Rejeitado (risco de bloqueio de GPU/telas azuis).
