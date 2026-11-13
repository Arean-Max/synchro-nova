# Synchro Nova — Sistem Değişiklikleri ve İnce Ayar Referansı

Bu belge, Synchro Nova tarafından yürütülen her sistem ayarının, kayıt defteri anahtarının ve komutun tam ve şeffaf bir dökümünü sağlar. Tüm eylemler kesinlikle yereldir, deterministiktir ve yürütülmeden önce yedeklenir.

---

## Güvenlik ve Geri Alma Garantisi

1. **Otomatik Yürütme Öncesi Yedekleme**:
Herhangi bir kayıt defteri anahtarını değiştirmeden önce, Synchro Nova bir sistem geri yükleme noktası oluşturur ve tüm hedef anahtarların mevcut durumunu "%APPDATA%\synchro\backups\tweak_safety_backup_<timestamp>.reg" içindeki bir ".reg" yedekleme dosyasına kaydeder.
2. **1 Tıklamayla Geri Alma**:
Uygulamada **"Откатить изменения"** (Geri Alma) seçeneğine tıklamak, Windows "reg.exe" aracılığıyla önceki ".reg" anlık görüntüsünü geri yükler ve mevcut değerleri yeniden okur.
3. **Güçlendirilmiş Komut Yürütme**:
Tüm yönetim araçları ("netsh", "powercfg", "sc", "bcdedit", "reg", "powershell") kesinlikle "%SystemRoot%\System32"den çözümlenir (yükseltilmiş PATH ele geçirmeyi önler) ve engellemeyen gözlemci zaman aşımlarıyla çalışır (yardımcı programlar için 5 saniye, sistem geri yükleme için 25 saniye).
4. **Sert Engelleme Listesi ve Hile Önleme Güvenliği**:
Güvenli olmayan işlemler (bellek bütünlüğünü/HVCI'yi devre dışı bırakmak, çekirdek zamanlayıcılarına yama uygulamak, disk belleği dosyasını devre dışı bırakmak veya rastgele sürücü geçersiz kılmaları uygulamak) motorda kalıcı olarak engellenir. Sıfır işlem enjeksiyonu veya bellek kancası, Easy Anti-Cheat, BattlEye, Vanguard ve VAC ile %100 uyumluluk sağlar.

---

## Tweaks Dökümünü Tamamla

### 1. Kullanıcı Düzeyinde Düzenlemeler (Standart Kullanıcı — Yönetici Gerekmez)

Bu ince ayarlar yalnızca 'HKEY_CURRENT_USER' kovanını değiştirir. Yükseltilmiş ayrıcalıklar gerektirmezler ve yalnızca etkin kullanıcı profilini etkilerler.

| Tweak ID | Hive & Path | Value / Command | Amaç |
| :--- | :--- | :--- | :--- |
| `gamebar-startup-off` | `HKCU\Software\Microsoft\GameBar` | `ShowStartupPanel = 0` | Oyun başlatıldığında Xbox Game Bar açılır penceresini önler |
| `pointer-precision-off` | `HKCU\Control Panel\Mouse` | `MouseSpeed = "0"`, `MouseThreshold1 = "0"`, `MouseThreshold2 = "0"` | Windows fare hızlandırmayı devre dışı bırakır (ham doğrusal giriş) |
| `advertising-id-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo` | `Enabled = 0` | Windows reklam tanımlayıcısını devre dışı bırakır |
| `tailored-experiences-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Privacy` | `TailoredExperiencesWithDiagnosticDataEnabled = 0` | Tanılama verilerine dayalı kişiselleştirilmiş ipuçlarını devre dışı bırakır |
| `clipboard-cloud-off` | `HKCU\Software\Microsoft\Clipboard` | `EnableClipboardHistory = 0`, `CloudClipboardAutomaticUpload = 0` | Panoyu tamamen yerel RAM'de tutar, bulut senkronizasyonunu devre dışı bırakır |
| `transparency-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` | `EnableTransparency = 0` | GPU belleğinden tasarruf etmek için DWM penceresi şeffaflığını devre dışı bırakır |
| `visual-effects-performance` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects` | `VisualFXSetting = 2` | En iyi görsel performans için Windows Gezgini'ni yapılandırır |
| `startup-delay-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize` | `StartupDelayInMSec = 0` | Yapay Windows masaüstü başlatma gecikmesini ortadan kaldırır |
| `menu-show-delay-low` | `HKCU\Control Panel\Desktop` | `MenuShowDelay = "10"` | Kullanıcı arayüzü menü animasyon gecikmesini 10 ms'ye düşürür |

---

### 2. Sistem Düzeyinde Düzenlemeler (Yönetici Ayrıcalıkları Gerektirir)

Bu ince ayarlar, sistem genelindeki politikaları yapılandırır ('HKEY_LOCAL_MACHINE') veya standart Windows yönetim araçlarını ('powercfg', 'netsh', 'fsutil') çağırır. Synchro bunları gerçekleştirmeden önce yüksekliği kontrol eder.

#### A. Oyun ve Multimedya Planlama (MMCSS)
| Tweak ID | Hive & Path | Value / Command | Amaç |
| :--- | :--- | :--- | :--- |
| `game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR`<br>`HKCU\System\GameConfigStore`<br>`HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR` | `AllowGameDVR = 0`<br>`GameDVR_Enabled = 0`<br>`AppCaptureEnabled = 0`, `HistoricalCaptureEnabled = 0` | Ücretsiz GPU kodlayıcıya arka plan Oyun DVR video yakalamayı devre dışı bırakır |
| `network-throttle-off` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `NetworkThrottlingIndex = 0xFFFFFFFF` | Yüksek öncelikli görevler sırasında Windows ağ azaltma mekanizmasını devre dışı bırakır |
| `system-responsiveness-10` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile` | `SystemResponsiveness = 10` | CPU zamanının %90'ını ön plandaki oyunlara ayırır (varsayılan %80'dir) |
| `mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games` | `GPU Priority = 8`, `Priority = 6`, `Scheduling Category = "High"`, `SFIO Priority = "High"` | Aktif oyunlar için multimedya zamanlayıcı görev önceliğini yükseltir |
| `hags-on` | `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `HwSchMode = 2` | Donanım Hızlandırmalı GPU Planlamayı Etkinleştirir (WDDM 2.7+) |
| `mpo-disable` | `HKLM\SOFTWARE\Microsoft\Windows\Dwm`<br>`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers` | `OverlayTestMode = 5`<br>`DisableOverlays = 1` | Çok Düzlemli Kaplamaları devre dışı bırakır (bazı GPU'lardaki siyah ekran/takılma sorunlarını çözer) |

#### B. Ağ Yığını Optimizasyonu ('netsh', 'ipconfig')
| Tweak ID | Tool / Command | Amaç |
| :--- | :--- | :--- |
| `dns-cache-flush` | `ipconfig /flushdns` | DNS çözümleyici önbelleğini temizler |
| `tcp-autotune-normal` | `netsh interface tcp set global autotuninglevel=normal` | Standart RFC 1323 TCP pencere ölçeklendirmesini geri yükler |
| `rss-on` | `netsh interface tcp set global rss=enabled` | Ağ paketi işlemeyi CPU çekirdekleri arasında dağıtmak için Alma Tarafı Ölçeklendirmesini etkinleştirir |
| `rsc-off` | `netsh interface tcp set global rsc=disabled` | Gerçek zamanlı oyunlarda gecikmeyi azaltmak için NIC'lerde Segment Birleştirme Alma'yı devre dışı bırakır |
| `ecn-off` | `netsh interface tcp set global ecncapability=disabled` | Eski yönlendiricilerde paketlerin düşmesini önlemek için Açık Tıkanıklık Bildirimini devre dışı bırakır |
| `flush-arp-cache` | `netsh interface ip delete arpcache` | Yerel Adres Çözümleme Protokolü tablosunu temizler |
| `winsock-reset` | `netsh winsock reset` | Windows ağ soketleri kataloğunu temiz duruma geri yükler |

#### C. Güç ve Depolama Yönetimi ('powercfg', 'fsutil')
| Tweak ID | Tool / Command | Amaç |
| :--- | :--- | :--- |
| `power-plan-high` | `powercfg /setactive SCHEME_MIN` | Standart Windows Yüksek Performanslı güç düzenini etkinleştirir |
| `ultimate-performance-plan` | `powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61` | Windows Ultimate Performance güç düzeninin kilidini açar ve etkinleştirir |
| `hibernate-off` | `powercfg /hibernate off` | Hazırda bekletme modunu devre dışı bırakır ve "hiberfil.sys"yi silerek disk alanını boşaltır |
| `ntfs-last-access-off` | `fsutil behavior set disableLastAccess 1` | Okunan her dosyada NTFS son erişim zaman damgalarının güncellenmesini durdurur |
| `trim-enable` | `fsutil behavior set DisableDeleteNotify 0` | SSD sağlığı ve aşınma seviyelendirmesi için TRIM bildirimlerinin etkin olmasını sağlar |
| `restore-point-first` | `powershell.exe Checkpoint-Computer -Description 'Synchro Pre-Tweak' ...` | Değişikliklerden önce tam bir Windows Sistem Geri Yükleme noktası oluşturur |

#### D. Sistem Telemetrisi ve Arka Plan Senkronizasyonu
| Tweak ID | Hive & Path | Value | Amaç |
| :--- | :--- | :--- | :--- |
| `activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System` | `EnableActivityFeed = 0`, `PublishUserActivities = 0`, `UploadUserActivities = 0` | Windows Zaman Çizelgesi ve etkinlik geçmişi yüklemelerini devre dışı bırakır |
| `delivery-optimization-lan` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` | `DODownloadMode = 1` | Windows Update Teslimat Optimizasyonunu yalnızca yerel LAN ile sınırlandırır (internete P2P yüklemesi yapılmaz) |

---

## Engellenen Güvenli Olmayan Düzenlemeler

Synchro, üçüncü taraf komut dosyalarında bulunan ve sistem bütünlüğünü veya hile önleme uyumluluğunu tehlikeye atan birkaç yaygın "agresif ayarı" kasıtlı olarak engeller:

- **Bellek Bütünlüğünün (HVCI) / Sanallaştırma Tabanlı Güvenliğin Devre Dışı Bırakılması**: Reddedildi.
- **Windows Defender'ı/Güvenlik Duvarını tamamen devre dışı bırakmak**: Reddedildi.
- **Disk Belleği Dosyası Devre Dışı Bırakılıyor (`pagefile.sys`)**: Reddedildi (bellek artışlarında oyunun çökmesine neden olur).
- **Dinamik Tick / Değişmez TSC kurcalamanın devre dışı bırakılması**: Reddedildi (saatlerin senkronizasyonu bozulabilir ve hile karşıtı buluşsal yöntemleri tetikleyebilir).
- **Toplu MSI (Mesaj Sinyalli Kesintiler) yardımcı programı korsanlığı**: Reddedildi (GPU tuğlalama/mavi ekran riski).
