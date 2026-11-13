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

### لماذا بنيت هذا

في كل مرة أقوم فيها بتشغيل CS2 أو Apex، كان عليّ فتح لوحة تحكم NVIDIA، وتشغيل Digital Vibrance لاكتشاف الأعداء في الزوايا المظلمة، ثم الاتصال به يدويًا مرة أخرى بعد إغلاق اللعبة حتى لا يحرق سطح المكتب والمتصفح شبكية العين.

يعمل Synchro على حل هذه المشكلة مرة واحدة وإلى الأبد:
- **تبديل اللون التلقائي**: قم بتشغيل لعبتك، وستبدأ الألوان. اضغط على Alt-Tab أو أغلق اللعبة، وستعود ألوان سطح المكتب على الفور إلى وضعها الطبيعي.
- **تنظيف الكمون**: يمنع تسريع ماوس Windows، ويحسن دقة المؤقت، ويقطع التخزين المؤقت لحزم الشبكة بحيث يبدو الإدخال فوريًا.
- **صفر انتفاخ**: تنزيل أقل من 5 ميغابايت، يبدأ بالمللي ثانية، ويبقى بهدوء في الدرج باستخدام حوالي 2 ميغابايت من ذاكرة الوصول العشوائي.

> [!TIP]
> **إعدادات Vibrance المسبقة التي تم اختبارها للألعاب الشائعة:**
> - **CS2 (الغبار II، السراب، الجحيم)**: اضبط على `150–165%`. تتوقف نماذج اللاعبين عن الاندماج في الجدران والصناديق.
> - **Valorant**: حوالي 135-140% تعتبر نسبة كافية. تحتوي اللعبة بالفعل على أبرز خطوط العدو؛ القيم الأعلى تغسل الألوان فقط.
> - **Apex Legends & Rust**: 160–170% يصنع العجائب في اكتشاف الصور الظلية في الأدغال وعلى مسافات طويلة.
> - **الشاشات المزدوجة**: ينطبق Vibrance فقط على الشاشة الأساسية التي تقوم بتشغيل لعبتك. تظل شاشتك الثانية مع Discord أو Chrome بألوان عادية.

---

### كيفية التنزيل والتشغيل (دليل المبتدئين)

إذا كنت جديدًا على GitHub، فإليك العملية برمتها:

1. انقر على [**الإصدارات**](https://github.com/Arean-Max/synchro-nova/releases) على الجانب الأيمن من هذه الصفحة.
2. قم بتنزيل **`synchro.exe`** — إنها النسخة المحمولة. احفظه في أي مكان وانقر نقرًا مزدوجًا لبدء التشغيل، دون الحاجة إلى برنامج تثبيت. (أو احصل على Synchro.Nova_*_x64-setup.exe إذا كنت تريد اختصارات قائمة ابدأ).
3. **إذا أظهر Windows نافذة SmartScreen منبثقة زرقاء ("Windows يحمي جهاز الكمبيوتر الخاص بك")**:
   - لا تقلق. يعرض Windows هذا لأي برنامج مستقل مجاني لا يشتري توقيع شركة بقيمة 500 دولار سنويًا من Microsoft.
   - انقر **"مزيد من المعلومات"** ➔ **"التشغيل على أية حال"**.
   - المشروع مفتوح المصدر 100% هنا. صفر قياس عن بعد، صفر عمال مناجم، صفر إعلانات.

---

### بداية سريعة: 3 خطوات بسيطة

1. قم بتشغيل "synchro.exe".
2. ضمن علامة التبويب **"اللون"**، اختر مستوى الحيوية لديك (على سبيل المثال `150%`).
3. حدد لعبتك من القائمة (أو أشر إلى ملف ".exe" الخاص بها).
تصغير إلى الدرج واللعب. لن تضطر أبدًا إلى تعديله يدويًا مرة أخرى.

---

### التعليمات

#### هل سيتم حظري في CS2 أو Valorant أو Rust أو Faceit؟
**100% لا. خطر الحظر صفر.**  
Synchro Nova **لا يلمس ذاكرة الوصول العشوائي (RAM) الخاصة باللعبة، ولا يُدخل ملفات DLL في العمليات، ولا يرسم تراكبات**. فهو يضبط جاما تمامًا من خلال واجهات برمجة تطبيقات عرض Windows القياسية - تمامًا مثل سحب شريط التمرير في لوحة التحكم NVIDIA أو AMD. أنظمة مكافحة الغش (VAC، Faceit، Vanguard، EAC، BattlEye) لا تعتبرها مجرد تفاعل في اللعبة.

#### لماذا تستخدم هذا عبر لوحة التحكم NVIDIA / AMD؟
لا يمكن للوحات تحكم GPU تبديل الإعدادات تلقائيًا لكل لعبة. إذا أضفت الحيوية هناك، فسيظل سطح المكتب بأكمله مشبعًا أثناء تصفح مقاطع الفيديو أو مشاهدتها. يقوم Synchro بمعالجة هذا تلقائيًا أثناء التنقل.

#### هل تعديلات Windows آمنة؟
نعم. نحن لا نلمس سلامة نظام التشغيل الأساسي، ولا نكسر Windows Defender، ولا نقوم بتعطيل ملف ترحيل الصفحات الخاص بك. قبل لمس أي إعداد، يقوم Synchro بإنشاء نقطة استعادة Windows ونسخة احتياطية للسجل. إذا لم يعجبك أي تعديل، فإن النقر على **"تراجع التغييرات"** يعيد كل شيء مرة أخرى على الفور.

---

### التوثيق والغوص العميق

للحصول على تفاصيل فنية متعمقة، تحقق من مواردنا المخصصة:
- 📖 [التوثيق التفاعلي الكامل على DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [مخطط الهندسة المعمارية على GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [القرص الكامل ومرجع التسجيل (TWEAKS_REFERENCE.ar.md)](docs/TWEAKS_REFERENCE.ar.md)

---

### البناء من المصدر (للمطورين)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       #طريقة التطوير
npm run build     # بناء الإصدار
```

---

### المساهمين

- [sotoykor-source](https://github.com/sotoykor-source) — المبدع الأصلي والمؤلف المنبع
- [Arean-Max](https://github.com/Arean-Max) — التطوير والصيانة

---

### رخصة

[MIT](LICENSE) — مجاني للاستخدام والتعديل والمشاركة.
