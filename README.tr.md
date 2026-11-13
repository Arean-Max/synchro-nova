<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Monitör renk doygunluğu ve Windows gecikme optimizasyonu için hafif bir masaüstü aracı.</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
  <a href="https://deepwiki.com/Arean-Max/synchro-nova"><img alt="Docs" src="https://img.shields.io/badge/Docs-DeepWiki-blueviolet?style=flat-square" /></a>
  <a href="https://gitdiagram.com/arean-max/synchro-nova"><img alt="Architecture" src="https://img.shields.io/badge/Diagram-GitDiagram-orange?style=flat-square" /></a>
  <a href="https://github.com/Arean-Max/synchro-nova/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" /></a>
  <a href="https://microsoft.com/windows"><img alt="Platform" src="https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square" /></a>
</p>

<p align="center">
  <a href="README.md">English</a> |
  <a href="README.zh.md">简体中文</a> |
  <a href="README.zht.md">繁體中文</a> |
  <a href="README.ko.md">한국어</a> |
  <a href="README.de.md">Deutsch</a> |
  <a href="README.es.md">Español</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.it.md">Italiano</a> |
  <a href="README.da.md">Dansk</a> |
  <a href="README.ja.md">日本語</a> |
  <a href="README.pl.md">Polski</a> |
  <a href="README.ru.md">Русский</a> |
  <a href="README.bs.md">Bosanski</a> |
  <a href="README.ar.md">العربية</a> |
  <a href="README.no.md">Norsk</a> |
  <a href="README.br.md">Português (Brasil)</a> |
  <a href="README.th.md">ไทย</a> |
  <a href="README.tr.md">Türkçe</a> |
  <a href="README.uk.md">Українська</a> |
  <a href="README.bn.md">বাংলা</a> |
  <a href="README.gr.md">Ελληνικά</a> |
  <a href="README.vi.md">Tiếng Việt</a>
</p>

[![Synchro Nova Preview](docs/screenshots/preview.png)](https://github.com/Arean-Max/synchro-nova/releases)

---

### Bunu neden inşa ettim

CS2 veya Apex'i her başlattığımda, NVIDIA Kontrol Panelini açmam, karanlık köşelerdeki düşmanları tespit etmek için Digital Vibrance'ı açmam ve oyunu kapattıktan sonra masaüstümün ve tarayıcımın retinalarımı yakmaması için manuel olarak tekrar kısmam gerekiyordu.

Synchro bunu kesin olarak çözüyor:
- **Otomatik renk değiştirme**: Oyununuzu başlatın, renkler devreye girer. Alt-Tab tuşuna bastığınızda veya oyunu kapattığınızda masaüstünüzün renkleri anında normale döner.
- **Gecikme temizleme**: Windows fare hızlandırmasını ortadan kaldırır, zamanlayıcı çözünürlüğünü optimize eder ve girişin anında hissedilmesi için ağ paketi arabelleğini keser.
- **Sıfır şişkinlik**: 5 MB'ın altında indirme, milisaniyeler içinde başlar ve ~2 MB RAM kullanarak tepside sessizce kalır.

> [!TIP]
> **Popüler Oyunlar için Test Edilmiş Titreşim Ön Ayarları:**
> - **CS2 (Dust II, Mirage, Inferno)**: "%150-165" olarak ayarlandı. Oyuncu modelleri duvarlara ve kasalara karışmayı bırakıyor.
> - **Valorant**: "%135-140" civarında oldukça fazla. Oyunda zaten düşmanın ana hatlarıyla ilgili önemli noktalar var; daha yüksek değerler yalnızca renkleri soluklaştırır.
> - **Apex Legends & Rust**: "%160–170", çalılıklardaki ve uzun mesafelerdeki silüetleri tespit etmede harikalar yaratır.
> - **Çift Monitör**: Titreşim yalnızca oyununuzu çalıştıran birincil monitör için geçerlidir. Discord veya Chrome'daki ikinci ekranınız normal renklerde kalıyor.

---

### Nasıl İndirilir ve Çalıştırılır (Başlangıç ​​Kılavuzu)

GitHub'ta yeniyseniz tüm süreç şu şekildedir:

1. Bu sayfanın sağ tarafındaki [**Sürümler**](https://github.com/Arean-Max/synchro-nova/releases) seçeneğine tıklayın.
2. **`synchro.exe`**'yi indirin — taşınabilir sürümdür. Herhangi bir yere kaydedin ve başlatmak için çift tıklayın, kurulum gerektirmez. (Veya Başlat menüsü kısayollarını istiyorsanız 'Synchro.Nova__*_x64-setup.exe'yi alın).
3. **Windows mavi bir SmartScreen açılır penceresi gösteriyorsa ("Windows PC'nizi korudu")**:
   - Merak etme. Windows bunu, Microsoft'tan yılda 500 ABD Doları tutarında kurumsal imza satın almayan tüm ücretsiz bağımsız programlar için gösterir.
   - **"Daha fazla bilgi"** ➔ **"Yine de çalıştır"** seçeneğini tıklayın.
   - Buradaki proje %100 açık kaynaktır. Sıfır telemetri, sıfır madenciler, sıfır reklam.

---

### Hızlı Başlangıç: 3 Basit Adım

1. 'Synchro.exe'yi başlatın.
2. **"Renk"** sekmesi altında canlılık düzeyinizi seçin (ör. "%150").
3. Listeden oyununuzu seçin (veya `.exe` dosyasının üzerine gelin).
Tepsiye küçültün ve oynatın. Bir daha asla manuel olarak ayarlamanıza gerek kalmayacak.

---

### SSS

#### CS2, Valorant, Rust veya Faceit'te yasaklanacak mıyım?
**%100 Hayır. Yasaklanma riski sıfırdır.**  
Synchro Nova **oyun RAM'ına dokunmaz, DLL'leri işlemlere eklemez ve katmanlar çizmez**. Gamayı tamamen standart Windows görüntüleme API'leri aracılığıyla ayarlar; tıpkı NVIDIA veya AMD kontrol panelindeki kaydırıcıyı sürüklemek gibi. Hile önleme sistemleri (VAC, Faceit, Vanguard, EAC, BattlEye) bunu bir oyun etkileşimi olarak bile görmüyor.

#### Bunu neden NVIDIA / AMD Kontrol Paneli üzerinden kullanmalısınız?
GPU kontrol panelleri oyun başına ayarları otomatik olarak değiştiremez. Burada titreşimi etkinleştirirseniz, videolara göz atarken veya izlerken masaüstünüzün tamamı aşırı doygun kalır. Synchro bunu anında otomatik olarak gerçekleştirir.

#### Windows ayarları güvenli mi?
Evet. Çekirdek işletim sistemi bütünlüğüne dokunmuyoruz, Windows Defender'ı bozmuyoruz ve sayfa dosyanızı devre dışı bırakmıyoruz. Synchro, herhangi bir ayara dokunmadan önce bir Windows Geri Yükleme Noktası ve bir kayıt defteri yedeği oluşturur. Bir ince ayarı beğenmezseniz **"Değişiklikleri Geri Al"** seçeneğini tıkladığınızda her şey anında geri alınır.

---

### Dokümantasyon ve Ayrıntılı İnceleme

Ayrıntılı teknik ayrıntılar için özel kaynaklarımıza göz atın:
- 📖 [DeepWiki'de Tam İnteraktif Belgeler](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram'da Mimari Diyagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Eksiksiz Düzenlemeler ve Kayıt Defteri Referansı (TWEAKS_REFERENCE.tr.md)](docs/TWEAKS_REFERENCE.tr.md)

---

### Kaynaktan derleme (geliştiriciler için)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # geliştirme modu
npm run build     # derlemeyi yayınla
```

---

### Katkıda Bulunanlar

- [sotoykor-source](https://github.com/sotoykor-source) — Orijinal yaratıcı ve yukarı akış yazarı
- [Arean-Max](https://github.com/Arean-Max) — Geliştirme ve bakım

---

### Lisans

[MIT](LICENSE) — kullanmak, değiştirmek ve paylaşmak ücretsizdir.
