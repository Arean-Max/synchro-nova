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

### Synchro Nova ne yapar?

Kısacası: oyunlarda renk canlılığını ve kontrastını artıran, düşmanların karanlık ortamlarda öne çıkmasına yardımcı olan hafif ve ücretsiz bir yardımcı programdır.

- **Otomatik Oyun Başına Profiller**: Oyununuz için favori titreşim seviyenizi bir kez ayarlayın. CS2, Valorant veya Apex'i başlattığınızda renkler otomatik olarak devreye girer. Alt sekme yaptığınızda veya çıktığınızda ekran renkleri hemen normale döner, böylece masaüstünüzde veya tarayıcınızda gözleriniz ağrımaz.
- **Gecikme ve Yanıt Verme**: Girişin hızlı olmasını sağlamak için test edilmiş, güvenli Windows yanıt verme ayarlarını (zamanlayıcı çözünürlüğü, ham fare girişi, ağ paketi işleme) içerir.
- **Sıfır Kaynak Sürüklemesi**: Uygulamanın tamamı 5 MB'ın altındadır, milisaniyeler içinde açılır ve yalnızca ~2 MB RAM kullanarak tepsinizde durur.

---

### Nasıl İndirilir ve Çalıştırılır (Başlangıç ​​Kılavuzu)

GitHub'u ilk kez ziyaret ediyorsanız tam olarak yapmanız gerekenler şunlardır:

1. Şuraya gidin: [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) sayfasına gidin (veya sağ kenar çubuğundaki "Yayınlar"ı tıklayın).
2. En son sürüm altında şunu indirin:
   - **`synchro.exe`** — **taşınabilir sürüm (önerilir)**. Herhangi bir klasöre indirin ve çalıştırmak için çift tıklayın. Kurulum gerekmez.
   - Veya **`Synchro.Nova_*_x64-setup.exe`** — Masaüstünü ve başlat menüsü kısayollarını tercih ediyorsanız standart Windows yükleyicisini kullanın.
3. **Windows mavi bir açılır pencere gösteriyorsa ("Windows PC'nizi korudu" / SmartScreen)**:
   - Bu, Microsoft'tan yılda 500 ABD doları tutarında pahalı bir kurumsal sertifika satın almayan tüm ücretsiz açık kaynak araçlarda gerçekleşir.
   - Tıklamak **"Daha fazla bilgi"**, sonra tıklayın **"Yine de koş"**.
   - Kaynak kodunun tamamı burada GitHub'da %100 açıktır; reklam yazılımı yok, telemetri yok, saçmalık yok.

---

### Hızlı Başlangıç ​​(1 Dakikalık Kurulum)

1. Öğle yemeği **Synchro Nova**.
2. Şuraya git: **"Renk"** sekmesine tıklayın ve titreşim düzeyinizi seçin (ör. `150%` veya `200%`).
3. Bunu oyununuza bağlayın (uygulama, çalışan oyunları otomatik olarak algılar veya `.exe` manuel olarak).
4. İşte bu! Tepsiye küçültün ve oynatın.

---

### SSS

#### CS2, Valorant, Rust veya Faceit'te yasaklanacak mıyım?
**%100 Hayır. Yasaklanma riski sıfırdır.**  
Synchro Nova **oyun belleğine dokunmaz, DLL'leri işlemlere eklemez ve oyun katmanlarını çizmez**. Masaüstü gammasını yalnızca standart Windows API'leri (DWM ve GPU sürücüleri) aracılığıyla ayarlar; tıpkı NVIDIA veya AMD kontrol panelindeki Digital Vibrance kaydırıcısını sürüklemek gibi. Hile önleme motorları (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) masaüstü renk değişikliklerini işaretlemez.

#### Neden sadece NVIDIA veya AMD Kontrol Panelini kullanmıyorsunuz?
GPU kontrol panelleri, oyunu başlattığınızda veya kapattığınızda renkleri otomatik olarak değiştirmez. Burada titreşimi artırırsanız, tüm Windows masaüstünüz ve tarayıcınız aşırı doymuş kalacaktır. Synchro bunu otomatik olarak anında gerçekleştirir: oyun akışı = zengin renkler; oyun kapandı = masaüstü renklerini temizle.

#### Sistem ayarları Windows'umu bozar mı?
Hayır. Tüm ince ayarlar standart, iyi belgelenmiş Windows parametrelerini kullanır. Synchro Nova, herhangi bir şeyi değiştirmeden önce bir Windows Geri Yükleme Noktası ve değiştirilen kayıt defteri anahtarlarının yedeğini oluşturur. seçeneğine tıklayarak her şeyi istediğiniz zaman Windows varsayılanlarına geri döndürebilirsiniz. **"Geri Alma Değişiklikleri"**.

---

### Dokümantasyon ve Ayrıntılı İnceleme

Bu sayfayı temiz ve okunması kolay tutmak için kapsamlı teknik belgeler özel wiki'mizde bulunmaktadır:
- 📖 [DeepWiki'de Tam İnteraktif Belgeler](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram'da Mimari Diyagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Eksiksiz Düzenlemeler ve Kayıt Defteri Referansı (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Kaynaktan derleme (geliştiriciler için)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Katkıda Bulunanlar

- [sotoykor-source](https://github.com/sotoykor-source) — Orijinal yaratıcı ve yukarı akış yazarı
- [Arean-Max](https://github.com/Arean-Max) — Geliştirme ve bakım

---

### Lisans

Altında dağıtılır [MIT License](LICENSE).
