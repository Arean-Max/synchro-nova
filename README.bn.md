<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">মনিটরের রঙের উজ্জ্বলতা ও Windows লেটেন্সি অপ্টিমাইজেশনের জন্য একটি হালকা ডেস্কটপ টুল।</p>

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

### কেন আমি এই নির্মাণ

যতবারই আমি CS2 বা Apex চালু করেছি, আমাকে NVIDIA কন্ট্রোল প্যানেল খুলতে হয়েছিল, অন্ধকার কোণে শত্রুদের চিহ্নিত করতে ডিজিটাল ভাইব্রেন্সকে ক্র্যাঙ্ক করতে হয়েছিল, এবং তারপরে গেমটি বন্ধ করার পরে ম্যানুয়ালি এটিকে আবার ডায়াল করতে হয়েছিল যাতে আমার ডেস্কটপ এবং ব্রাউজার আমার রেটিনা বার্ন না করে।

Synchro এটি একবার এবং সব জন্য সমাধান করে:
- **স্বয়ংক্রিয় রঙ স্যুইচিং**: আপনার গেমটি চালু করুন, রঙগুলি শুরু করুন৷ Alt-Tab বা গেমটি বন্ধ করুন, আপনার ডেস্কটপের রঙগুলি অবিলম্বে স্বাভাবিক অবস্থায় ফিরে আসবে৷
- **লেটেন্সি ক্লিনআপ**: উইন্ডোজ মাউসের ত্বরণকে মেরে ফেলে, টাইমার রেজোলিউশনকে অপ্টিমাইজ করে এবং নেটওয়ার্ক প্যাকেট বাফারিংকে ছাঁটাই করে যাতে ইনপুট তাৎক্ষণিক অনুভূত হয়।
- **শূন্য ফোলা**: 5 MB এর নিচে ডাউনলোড, মিলিসেকেন্ডে শুরু হয় এবং ~2 MB RAM ব্যবহার করে ট্রেতে চুপচাপ বসে থাকে।

> [!TIP]
> **জনপ্রিয় গেমগুলির জন্য পরীক্ষিত ভাইব্রেন্স প্রিসেট:**
> - **CS2 (Dust II, Mirage, Inferno)**: `150–165%` এ সেট করুন। প্লেয়ার মডেল দেয়াল এবং crates মধ্যে মিশ্রিত বন্ধ.
> - **বীর্যপূর্ণ**: প্রায় `135-140%` প্রচুর। গেমটিতে ইতিমধ্যে শত্রুর রূপরেখা হাইলাইট রয়েছে; উচ্চ মান শুধু রং ধোয়া আউট.
> - **Apex Legends & Rust**: `160–170%` ঝোপের মধ্যে এবং দীর্ঘ দূরত্বে সিলুয়েট দেখার জন্য বিস্ময়কর কাজ করে।
> - **দ্বৈত মনিটর**: ভাইব্রেন্স শুধুমাত্র আপনার গেমটি চালানো প্রাথমিক মনিটরের ক্ষেত্রে প্রযোজ্য। ডিসকর্ড বা ক্রোমের সাথে আপনার দ্বিতীয় স্ক্রিন স্বাভাবিক রঙে থাকে।

---

### কিভাবে ডাউনলোড করবেন এবং চালাবেন (শিশুর নির্দেশিকা)

আপনি যদি GitHub-এ নতুন হন, তাহলে পুরো প্রক্রিয়াটি এখানে রয়েছে:

1. এই পৃষ্ঠার ডানদিকে [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) এ ক্লিক করুন।
2. ডাউনলোড করুন **`synchro.exe`** — এটি পোর্টেবল সংস্করণ। এটিকে যেকোনো জায়গায় সংরক্ষণ করুন এবং চালু করতে ডাবল-ক্লিক করুন, কোনো ইনস্টলারের প্রয়োজন নেই৷ (বা আপনি যদি স্টার্ট মেনু শর্টকাট চান তবে `Synchro.Nova__*_x64-setup.exe` ধরুন)।
3. **যদি উইন্ডোজ একটি নীল স্মার্টস্ক্রিন পপআপ দেখায় ("উইন্ডোজ আপনার পিসি সুরক্ষিত করেছে")**:
   - চিন্তা করবেন না। Microsoft থেকে $500/বছর কর্পোরেট স্বাক্ষর ক্রয় করে না এমন যেকোনো বিনামূল্যের ইন্ডি প্রোগ্রামের জন্য Windows এটি দেখায়।
   - **"আরো তথ্য"** ➔ **"যেভাবেই চালান"** ক্লিক করুন।
   - প্রকল্পটি এখানে 100% ওপেন সোর্স। জিরো টেলিমেট্রি, শূন্য মাইনার, শূন্য বিজ্ঞাপন।

---

### দ্রুত শুরু: 3টি সহজ ধাপ

1. `synchro.exe` চালু করুন।
2. **"রঙ"** ট্যাবের অধীনে, আপনার ভাইব্রেন্স লেভেল বেছে নিন (যেমন `150%`)।
3. তালিকা থেকে আপনার গেম নির্বাচন করুন (বা এর `.exe` ফাইলে নির্দেশ করুন)।
ট্রে এবং খেলতে ছোট করুন। আপনাকে আর কখনো ম্যানুয়ালি অ্যাডজাস্ট করতে হবে না।

---

### FAQ

#### আমি কি CS2, Valorant, Rust, বা Faceit-এ নিষিদ্ধ হব?
**100% না। নিষেধাজ্ঞার ঝুঁকি শূন্য।**  
Synchro Nova **গেম র‍্যাম স্পর্শ করে না, প্রক্রিয়াগুলিতে DLL ইনজেক্ট করে না এবং ওভারলে আঁকে না**. এটি স্ট্যান্ডার্ড উইন্ডোজ ডিসপ্লে API-এর মাধ্যমে সম্পূর্ণরূপে গামা সামঞ্জস্য করে — ঠিক যেমন NVIDIA বা AMD কন্ট্রোল প্যানেলে স্লাইডার টেনে আনার মতো। অ্যান্টি-চিট সিস্টেম (VAC, Faceit, Vanguard, EAC, BattlEye) এমনকি এটিকে একটি গেম ইন্টারঅ্যাকশন হিসেবেও দেখে না।

#### NVIDIA/AMD কন্ট্রোল প্যানেলে কেন এটি ব্যবহার করবেন?
GPU কন্ট্রোল প্যানেল স্বয়ংক্রিয়ভাবে প্রতি গেমের সেটিংস টগল করতে পারে না। আপনি যদি সেখানে ভাইব্রেন্স ক্র্যাঙ্ক করেন, তাহলে ভিডিও ব্রাউজ করার সময় বা দেখার সময় আপনার পুরো ডেস্কটপ অত্যধিক স্যাচুরেটেড থাকে। ফ্লাইতে সিঙ্ক্রো এটি স্বয়ংক্রিয়ভাবে পরিচালনা করে।

#### উইন্ডোজ tweaks নিরাপদ?
হ্যাঁ। আমরা মূল OS অখণ্ডতা স্পর্শ করি না, আমরা Windows Defender ভাঙি না এবং আমরা আপনার পেজফাইল অক্ষম করি না। কোনো সেটিং স্পর্শ করার আগে, Synchro একটি Windows Restore Point এবং একটি রেজিস্ট্রি ব্যাকআপ তৈরি করে। আপনি যদি একটি টুইক পছন্দ না করেন, তাহলে **"রোলব্যাক চেঞ্জেস"** ক্লিক করলে সবকিছু অবিলম্বে ফিরে আসবে।

---

### ডকুমেন্টেশন এবং গভীর ডুব

গভীর প্রযুক্তিগত বিবরণের জন্য, আমাদের উত্সর্গীকৃত সংস্থানগুলি দেখুন:
- 📖 [ডিপউইকিতে সম্পূর্ণ ইন্টারেক্টিভ ডকুমেন্টেশন](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [গিটডায়াগ্রামে আর্কিটেকচার ডায়াগ্রাম](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [সম্পূর্ণ টুইক এবং রেজিস্ট্রি রেফারেন্স (TWEAKS_REFERENCE.bn.md)](docs/TWEAKS_REFERENCE.bn.md)

---

### উৎস থেকে তৈরি করুন (ডেভেলপারদের জন্য)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # উন্নয়ন মোড
npm run build     # রিলিজ বিল্ড
```

---

### অবদানকারী

- [sotoykor-source](https://github.com/sotoykor-source) — মূল স্রষ্টা এবং আপস্ট্রিম লেখক
- [Arean-Max](https://github.com/Arean-Max) — উন্নয়ন ও রক্ষণাবেক্ষণ

---

### লাইসেন্স

[MIT](LICENSE) — ব্যবহার, পরিবর্তন, এবং ভাগ বিনামূল্যে.
