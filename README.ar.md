<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">أداة خفيفة لتحسين تشبع ألوان الشاشة وتقليل زمن استجابة نظام Windows.</p>

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

### التثبيت

قم بتنزيل أحدث إصدار من صفحة [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **نسخة محمولة (`Synchro-Nova-*-Portable.zip`)** — فك الضغط وشغّل `synchro.exe` مباشرة بدون تثبيت.
- **برنامج التثبيت (`Synchro.Nova_*_x64-setup.exe`)** — مثبت نظام Windows القياسي مع اختصارات سطح المكتب.

---

### الأسئلة الشائعة (FAQ)

#### ما هي هذه الأداة؟
برنامج خفيف للاعبين. يمنح الألعاب ألواناً حيوية، ويحسن رؤية الأهداف، ويقلل من تأخر استجابة الماوس في Windows. يتم التبديل التلقائي للملفات عند بدء اللعبة.

#### هل يسبب حظراً في الألعاب (Ban)؟
كلا، آمن 100%. لا يقترب البرنامج من ذاكرة اللعبة ولا يقوم بحقن ملفات أو إظهار شاشات فوقية. يتم تعديل الألوان عبر واجهة WinAPI الرسمية مثل لوحات NVIDIA أو AMD تماماً. متوافق مع VAC و Faceit و Vanguard و EAC و BattlEye.

#### ما الذي يميزه عن VibranceGUI؟
مبني بلغة Rust مع Tauri: تشغيل فوري، واستهلاك 0% من المعالج وحوالي 4 ميجابايت من الذاكرة فقط في شريط المهام. يدعم التبديل التلقائي للملفات لكل لعبة وتعديلات آمنة لاستجابة النظام.

#### هل يمكن للتعديلات إلحاق الضرر بنظام Windows؟
كلا. قبل إجراء أي تعديل، يقوم البرنامج بإنشاء نقطة استعادة للنظام ونسخة احتياطية للريجستري تلقائياً.

---

### البناء من المصدر

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # تشغيل في وضع التطوير
npm run build     # بناء برنامج التثبيت النهائي
```

---

### الترخيص

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

