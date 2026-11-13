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

### ทำไมฉันถึงสร้างสิ่งนี้

ทุกครั้งที่ฉันเปิดตัว CS2 หรือ Apex ฉันจะต้องเปิด NVIDIA Control Panel เพิ่ม Digital Vibrance ขึ้นเพื่อตรวจจับศัตรูในมุมมืด จากนั้นหมุนกลับด้วยตนเองหลังจากปิดเกม เพื่อให้เดสก์ท็อปและเบราว์เซอร์ของฉันไม่ทำให้จอประสาทตาไหม้

Synchro แก้ปัญหานี้ทันทีและตลอดไป:
- **การเปลี่ยนสีอัตโนมัติ**: เริ่มเกมของคุณ สีต่างๆ จะเริ่มทำงาน กด Alt-Tab หรือปิดเกม สีเดสก์ท็อปของคุณจะกลับมาเป็นปกติทันที
- **การล้างข้อมูลแฝง**: ฆ่าการเร่งความเร็วเมาส์ของ Windows ปรับความละเอียดของตัวจับเวลาให้เหมาะสม และลดการบัฟเฟอร์แพ็กเก็ตเครือข่าย เพื่อให้รู้สึกเหมือนป้อนข้อมูลได้ทันที
- **บวมเป็นศูนย์**: การดาวน์โหลดต่ำกว่า 5 MB เริ่มต้นในหน่วยมิลลิวินาที และอยู่ในถาดอย่างเงียบๆ โดยใช้ RAM ประมาณ 2 MB

> [!TIP]
> **ค่าที่ตั้งไว้ล่วงหน้าของ Vibrance ที่ทดสอบแล้วสำหรับเกมยอดนิยม:**
> - **CS2 (ฝุ่น II, มิราจ, นรก)**: ตั้งค่าเป็น `150–165%` โมเดลผู้เล่นหยุดผสมเข้ากับผนังและลังไม้
> - **Valorant**: ประมาณ `135–140%` ก็เพียงพอแล้ว เกมดังกล่าวมีไฮไลท์โครงร่างของศัตรูอยู่แล้ว ค่าที่สูงกว่าเพียงแค่ล้างสีออก
> - **Apex Legends & Rust**: `160–170%` ใช้งานได้อย่างมหัศจรรย์ในการมองเห็นเงาในพุ่มไม้และในระยะไกล
> - **จอภาพคู่**: ความสั่นสะเทือนมีผลเฉพาะกับจอภาพหลักที่กำลังเล่นเกมของคุณเท่านั้น หน้าจอที่สองของคุณที่มี Discord หรือ Chrome ยังคงเป็นสีปกติ

---

### วิธีดาวน์โหลดและเรียกใช้ (คู่มือสำหรับผู้เริ่มต้น)

หากคุณเพิ่งเริ่มใช้ GitHub นี่คือกระบวนการทั้งหมด:

1. คลิก [**เผยแพร่**](https://github.com/Arean-Max/synchro-nova/releases) ทางด้านขวาของหน้านี้
2. ดาวน์โหลด **`synchro.exe`** — เป็นเวอร์ชันพกพา บันทึกได้ทุกที่และดับเบิลคลิกเพื่อเปิดใช้งาน ไม่จำเป็นต้องติดตั้ง (หรือคว้า `Synchro.Nova_*_x64-setup.exe` หากคุณต้องการทางลัดเมนู Start)
3. **หาก Windows แสดงป๊อปอัป SmartScreen สีน้ำเงิน ("Windows ป้องกันพีซีของคุณ")**:
   - ไม่ต้องกังวล. Windows จะแสดงสิ่งนี้สำหรับโปรแกรมอินดี้ฟรีที่ไม่ได้ซื้อลายเซ็นองค์กรมูลค่า 500 ดอลลาร์สหรัฐฯ ต่อปีจาก Microsoft
   - คลิก **"ข้อมูลเพิ่มเติม"** ➔ **"เรียกใช้ต่อไป"**
   - โครงการนี้เป็นโอเพ่นซอร์ส 100% ที่นี่ การวัดและส่งข้อมูลทางไกลเป็นศูนย์, นักขุดเป็นศูนย์, ไม่มีโฆษณา

---

### เริ่มต้นอย่างรวดเร็ว: 3 ขั้นตอนง่ายๆ

1. เปิดตัว `synchro.exe`
2. ใต้แท็บ **"สี"** ให้เลือกระดับความสั่นสะเทือน (เช่น `150%`)
3. เลือกเกมของคุณจากรายการ (หรือชี้ไปที่ไฟล์ `.exe`)
ย่อเล็กสุดไปที่ถาดและเล่น คุณไม่จำเป็นต้องปรับด้วยตนเองอีกต่อไป

---

### คำถามที่พบบ่อย

#### ฉันจะถูกแบนใน CS2, Valorant, Rust หรือ Faceit หรือไม่?
**100% ไม่ ความเสี่ยงในการแบนเป็นศูนย์**  
Synchro Nova **ไม่สัมผัส RAM ของเกม ไม่ฉีด DLL เข้าไปในกระบวนการ และไม่วาดโอเวอร์เลย์**. มันปรับแกมมาผ่าน API การแสดงผลของ Windows มาตรฐาน — เหมือนกับการลากแถบเลื่อนในแผงควบคุม NVIDIA หรือ AMD ระบบป้องกันการโกง (VAC, Faceit, Vanguard, EAC, BattlEye) ไม่ได้มองว่าเป็นการโต้ตอบในเกมด้วยซ้ำ

#### เหตุใดจึงใช้สิ่งนี้ผ่านแผงควบคุม NVIDIA / AMD
แผงควบคุม GPU ไม่สามารถสลับการตั้งค่าต่อเกมได้โดยอัตโนมัติ หากคุณเปลี่ยนความสั่นสะเทือนตรงนั้น เดสก์ท็อปทั้งหมดของคุณจะยังคงมีความอิ่มตัวมากเกินไปขณะเรียกดูหรือดูวิดีโอ Synchro จัดการสิ่งนี้โดยอัตโนมัติทันที

#### การปรับแต่ง Windows ปลอดภัยหรือไม่?
ใช่. เราไม่แตะต้องความสมบูรณ์ของระบบปฏิบัติการหลัก เราไม่ทำลาย Windows Defender และเราไม่ปิดการใช้งานไฟล์เพจของคุณ ก่อนที่จะแตะการตั้งค่าใดๆ Synchro จะสร้าง Windows Restore Point และการสำรองข้อมูลรีจิสทรี หากคุณไม่ชอบการปรับแต่งใดๆ การคลิก **"ย้อนกลับการเปลี่ยนแปลง"** จะคืนค่าทุกอย่างกลับคืนทันที

---

### เอกสารประกอบและเจาะลึก

สำหรับรายละเอียดทางเทคนิคเชิงลึก โปรดดูแหล่งข้อมูลเฉพาะของเรา:
- 📖 [เอกสารเชิงโต้ตอบฉบับสมบูรณ์บน DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [แผนภาพสถาปัตยกรรมบน GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [ปรับแต่งและอ้างอิงรีจิสทรีให้สมบูรณ์ (TWEAKS_REFERENCE.th.md)](docs/TWEAKS_REFERENCE.th.md)

---

### สร้างจากแหล่งที่มา (สำหรับนักพัฒนา)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       #โหมดการพัฒนา
npm run build     #ปล่อยบิลด์
```

---

### ผู้ร่วมให้ข้อมูล

- [sotoykor-source](https://github.com/sotoykor-source) — ผู้สร้างดั้งเดิมและผู้เขียนต้นน้ำ
- [Arean-Max](https://github.com/Arean-Max) — การพัฒนาและการบำรุงรักษา

---

### ใบอนุญาต

[MIT](LICENSE) — ใช้งาน แก้ไข และแบ่งปันได้ฟรี
