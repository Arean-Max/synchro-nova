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

### ইনস্টলেশন

[**Releases**](https://github.com/Arean-Max/synchro-nova/releases) পেজ থেকে সর্বশেষ সংস্করণটি ডাউনলোড করুন:

- **পোর্টেবল সংস্করণ (`Synchro-Nova-*-Portable.zip`)** — আনজিপ করে `synchro.exe` চালু করুন। কোনো ইনস্টলেশনের প্রয়োজন নেই।
- **ইনস্টলার (`Synchro.Nova_*_x64-setup.exe`)** — শর্টকাট ও আনইনস্টলার সহ সাধারণ Windows সেটআপ।

---

### সাধারণ জিজ্ঞাসা (FAQ)

#### এটি কী ধরণের সফটওয়্যার?
গেমারদের জন্য তৈরি একটি হালকা ইউটিলিটি। এটি গেমে রঙকে স্পষ্ট করে, শত্রু শনাক্ত করতে সহায়তা করে এবং Windows এর অপ্রয়োজনীয় ইনপুট ল্যাগ হ্রাস করে। গেম চালু হলে স্বয়ংক্রিয়ভাবে প্রোফাইল সক্রিয় হয়।

#### এতে কি গেম থেকে ব্যান হওয়ার ঝুঁকি আছে?
না, একদম নিরাপদ। Synchro Nova গেমের মেমোরিতে হস্তক্ষেপ করে না, কোনো ফাইল ইনজেক্ট করে না এবং ওভারলে ব্যবহার করে না। এটি NVIDIA বা AMD প্যানেলের মতোই সাধারণ Windows WinAPI দিয়ে ডিসপ্লে পরিচালনা করে। VAC, Faceit, Vanguard, EAC এবং BattlEye এর জন্য সম্পূর্ণ নিরাপদ।

#### VibranceGUI বা GPU ড্রাইভার প্যানেলের চেয়ে এটি কেন ভালো?
Rust + Tauri দিয়ে তৈরি: তাৎক্ষণিকভাবে চালু হয়, ব্যাকগ্রাউন্ডে 0% CPU ও মাত্র ~4 MB RAM ব্যবহার করে। এতে স্বয়ংক্রিয় গেম প্রোফাইল স্যুইচিং এবং নিরাপদ সিস্টেম টিউনিং সুবিধা রয়েছে।

#### সিস্টেম টুইকগুলো কি Windows নষ্ট করতে পারে?
না। যেকোনো পরিবর্তন করার আগে প্রোগ্রামটি স্বয়ংক্রিয়ভাবে একটি Windows Restore Point ও রেজিস্ট্রি ব্যাকআপ তৈরি করে, ফলে যেকোনো সময় এক ক্লিকে পূর্বাবস্থায় ফেরা যায়।

---

### সোর্স কোড থেকে বিল্ড

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # ডেভেলপমেন্ট মোডে চালান
npm run build     # রিলিজ ইনস্টলার বিল্ড করুন
```

---

### লাইসেন্স

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
