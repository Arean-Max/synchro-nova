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

### ซิงโครโนวาทำอะไร?

กล่าวโดยย่อ: มันเป็นยูทิลิตี้ฟรีน้ำหนักเบาที่เพิ่มความมีชีวิตชีวาของสีและคอนทราสต์ในเกม ช่วยให้ศัตรูโดดเด่นในสภาพแวดล้อมที่มืด

- **โปรไฟล์ต่อเกมอัตโนมัติ**: ตั้งค่าระดับความสั่นสะเทือนที่คุณชื่นชอบสำหรับเกมของคุณเพียงครั้งเดียว เมื่อคุณเปิด CS2, Valorant หรือ Apex สีต่างๆ จะถูกเตะโดยอัตโนมัติ เมื่อคุณกด Alt Tab หรือออก สีที่แสดงของคุณจะกลับมาเป็นปกติทันที เพื่อให้ดวงตาของคุณไม่เจ็บเมื่ออยู่บนเดสก์ท็อปหรือในเบราว์เซอร์
- **ความหน่วงและการตอบสนอง**: รวมการปรับแต่งการตอบสนองของ Windows ที่ผ่านการทดสอบและปลอดภัย (ความละเอียดของตัวจับเวลา การป้อนข้อมูลเมาส์ดิบ การจัดการแพ็กเก็ตเครือข่าย) เพื่อให้การป้อนข้อมูลรวดเร็ว
- **การลากทรัพยากรเป็นศูนย์**: แอปทั้งหมดมีขนาดไม่เกิน 5 MB เปิดได้ในหน่วยมิลลิวินาที และอยู่ในถาดของคุณโดยใช้ RAM เพียง ~2 MB

---

### วิธีดาวน์โหลดและเรียกใช้ (คู่มือสำหรับผู้เริ่มต้น)

หากคุณกำลังเยี่ยมชม GitHub เป็นครั้งแรก นี่คือสิ่งที่ต้องทำ:

1. มุ่งหน้าไปที่ [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) หน้า (หรือคลิก "เผยแพร่" ที่แถบด้านข้างขวา)
2. ภายใต้รุ่นล่าสุด ให้ดาวน์โหลด:
   - **`synchro.exe`** — **รุ่นพกพา (แนะนำ)**. เพียงดาวน์โหลดลงในโฟลเดอร์ใดก็ได้แล้วดับเบิลคลิกเพื่อเรียกใช้ ไม่จำเป็นต้องตั้งค่า
   - หรือ **`Synchro.Nova_*_x64-setup.exe`** — ตัวติดตั้ง Windows มาตรฐาน หากคุณต้องการทางลัดบนเดสก์ท็อปและเมนูเริ่ม
3. **หาก Windows แสดงป๊อปอัปสีน้ำเงิน ("Windows ป้องกันพีซีของคุณ" / SmartScreen)**:
   - สิ่งนี้เกิดขึ้นกับเครื่องมือโอเพ่นซอร์สฟรีทั้งหมดที่ไม่ได้ซื้อใบรับรององค์กรราคาแพงจาก Microsoft มูลค่า 500 ดอลลาร์สหรัฐฯ ต่อปี
   - คลิก **"ข้อมูลเพิ่มเติม"**, จากนั้นคลิก **"วิ่งยังไงก็ได้"**.
   - ซอร์สโค้ดทั้งหมดเปิดอยู่ที่นี่ 100% บน GitHub — ไม่มีแอดแวร์ ไม่มีการตรวจวัดระยะไกล และไร้สาระ

---

### เริ่มต้นอย่างรวดเร็ว (การตั้งค่า 1 นาที)

1. ปล่อย **Synchro Nova**.
2. ไปที่ **"สี"** แท็บและเลือกระดับความสั่นสะเทือนของคุณ (เช่น `150%` หรือ `200%`).
3. เชื่อมโยงกับเกมของคุณ (แอปจะตรวจจับเกมที่กำลังรันอยู่โดยอัตโนมัติ หรือคุณสามารถเลือกได้ `.exe` ด้วยตนเอง)
4. แค่นั้นแหละ! ย่อเล็กสุดไปที่ถาดแล้วเล่น

---

### คำถามที่พบบ่อย

#### ฉันจะถูกแบนใน CS2, Valorant, Rust หรือ Faceit หรือไม่?
**100% ไม่ ความเสี่ยงในการแบนเป็นศูนย์**  
Synchro Nova **ไม่สัมผัสหน่วยความจำเกม ไม่ฉีด DLL เข้าไปในกระบวนการ และไม่วาดโอเวอร์เลย์เกม**. โดยจะปรับแกมมาเดสก์ท็อปผ่าน Windows API มาตรฐานเท่านั้น (ไดรเวอร์ DWM และ GPU) — เหมือนกับการลากแถบเลื่อน Digital Vibrance ในแผงควบคุม NVIDIA หรือ AMD เอ็นจิ้นต่อต้านการโกง (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) จะไม่ตั้งค่าสถานะการเปลี่ยนแปลงสีของเดสก์ท็อป

#### ทำไมไม่ใช้เพียง NVIDIA หรือ AMD Control Panel?
แผงควบคุม GPU จะไม่สลับสีโดยอัตโนมัติเมื่อคุณเปิดหรือปิดเกม หากคุณเพิ่มความสั่นสะเทือนที่นั่น เดสก์ท็อป Windows และเบราว์เซอร์ทั้งหมดของคุณจะยังคงมีความอิ่มตัวมากเกินไป ซิงโครจะจัดการสิ่งนี้โดยอัตโนมัติทันที: การรันเกม = สีสันที่หลากหลาย; เกมปิด = ล้างสีเดสก์ท็อป

#### ระบบจะปรับแต่งจะทำให้ Windows ของฉันยุ่งหรือไม่
ไม่ การปรับแต่งทั้งหมดใช้พารามิเตอร์ Windows มาตรฐานที่มีการบันทึกไว้อย่างดี ก่อนที่จะแก้ไขสิ่งใดๆ Synchro Nova จะสร้าง Windows Restore Point และสำรองข้อมูลของรีจิสตรีคีย์ที่แก้ไข คุณสามารถเปลี่ยนทุกอย่างกลับเป็นค่าเริ่มต้นของ Windows ได้ตลอดเวลาโดยคลิก **"การเปลี่ยนแปลงย้อนกลับ"**.

---

### เอกสารประกอบและเจาะลึก

เพื่อให้หน้านี้สะอาดและอ่านง่าย เอกสารทางเทคนิคที่ครอบคลุมจึงอยู่ในวิกิเฉพาะของเรา:
- 📖 [เอกสารเชิงโต้ตอบฉบับสมบูรณ์บน DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [แผนภาพสถาปัตยกรรมบน GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [ปรับแต่งและอ้างอิงรีจิสทรีให้สมบูรณ์ (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### สร้างจากแหล่งที่มา (สำหรับนักพัฒนา)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### ผู้ร่วมให้ข้อมูล

- [sotoykor-source](https://github.com/sotoykor-source) — ผู้สร้างดั้งเดิมและผู้เขียนต้นน้ำ
- [Arean-Max](https://github.com/Arean-Max) — การพัฒนาและการบำรุงรักษา

---

### ใบอนุญาต

จัดจำหน่ายภายใต้ [MIT License](LICENSE).
