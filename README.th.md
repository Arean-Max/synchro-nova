<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">ยูทิลิตี้ขนาดเล็กสำหรับปรับสีจอภาพและลดความหน่วงบน Windows สำหรับคอเกม</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
  <a href="https://github.com/Arean-Max/synchro-nova/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-AGPL--3.0-blue?style=flat-square" /></a>
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

### การติดตั้ง

ดาวน์โหลดเวอร์ชันล่าสุดได้ที่หน้า [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **เวอร์ชันพกพา (`Synchro-Nova-*-Portable.zip`)** — แตกไฟล์แล้วเปิด `synchro.exe` ได้ทันทีโดยไม่ต้องติดตั้ง
- **ตัวติดตั้ง (`Synchro.Nova_*_x64-setup.exe`)** — ตัวติดตั้งมาตรฐานสำหรับ Windows พร้อมทางลัดและตัวถอนการติดตั้ง

---

### คำถามที่พบบ่อย (FAQ)

#### โปรแกรมนี้คืออะไร?
โปรแกรมขนาดเบาสำหรับเกมเมอร์ ช่วยให้สีสันในเกมสดใส ชัดเจนขึ้น ช่วยให้มองเห็นศัตรูได้ชัดเจนยิ่งขึ้น และช่วยลดอาการหน่วง (Input Lag) ของ Windows สลับโปรไฟล์อัตโนมัติเมื่อเข้าและออกจากเกม

#### จะโดนแบนจากระบบแอนตี้ชีตไหม?
ไม่โดนแน่นอน 100% โปรแกรมไม่แตะต้องหน่วยความจำของเกม ไม่แทรกไฟล์ DLL และไม่มีการใช้อินเตอร์เฟซทับซ้อน ปรับสีผ่าน WinAPI มาตรฐานของระบบ เช่นเดียวกับแผงควบคุมของ NVIDIA หรือ AMD จึงปลอดภัยต่อ VAC, Faceit, Vanguard, EAC และ BattlEye

#### ดีกว่า VibranceGUI หรือแผงไดรเวอร์อย่างไร?
เขียนขึ้นด้วย Rust + Tauri: เปิดเร็วมาก กิน CPU 0% และใช้แรมเพียง ~4 MB ในพื้นหลัง นอกจากการปรับความสดของสีแล้ว ยังมีระบบสลับโปรไฟล์ตามเกมอัตโนมัติและการปรับแต่งระบบที่ปลอดภัย

#### การปรับแต่งระบบจะทำให้ Windows เสียหายหรือไม่?
ไม่เสียหาย โปรแกรมจะสร้างจุดคืนค่าระบบ (System Restore Point) และสำรอง Registry ให้โดยอัตโนมัติก่อนที่จะเปลี่ยนแปลงค่าใดๆ ทำให้กู้คืนได้ทุกเมื่อในคลิกเดียว

---

### การคอมไพล์จากซอร์สโค้ด

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # รันในโหมดพัฒนา
npm run build     # คอมไพล์ตัวติดตั้ง
```

---

### สัญญาอนุญาต

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

