<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">輕量級桌面工具，用於螢幕色彩飽和度增強與 Windows 延遲優化。</p>

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

### 安裝

從 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) 下載最新版本：

- **免安裝版 (`Synchro-Nova-*-Portable.zip`)** — 解壓縮即可執行 `synchro.exe`，無需安裝。
- **安裝程式 (`Synchro.Nova_*_x64-setup.exe`)** — 標準 Windows 安裝程式，含捷徑與解除安裝功能。

---

### 常見問題 (FAQ)

#### 這是什麼工具？
專為競技遊戲打造的輕巧工具。讓畫面色彩更鮮明、提升目標能見度，並消除 Windows 額外延遲。啟動遊戲時自動切換設定，退出後自動還原。

#### 會被防作弊系統封鎖嗎？
不會。Synchro Nova 不讀寫遊戲記憶體、不注入 DLL、不繪製懸浮窗。完全使用原生 Windows WinAPI 調整螢幕輸出，對 VAC、Faceit、Vanguard、EAC 與 BattlEye 100% 安全。

#### 比起 VibranceGUI 或顯卡控制面板好在哪裡？
採用 Rust + Tauri 架構：極速啟動，背景常駐僅耗用約 4 MB 記憶體及 0% CPU。支援遊戲專屬自動設定切換與安全的系統延遲調校。

#### 系統優化會損壞 Windows 嗎？
不會。所有調校皆經測試驗證，且在套用前會自動建立系統還原點與登錄檔備份，隨時可一鍵還原。

---

### 從原始碼編譯

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 以開發模式啟動
npm run build     # 編譯發行安裝檔
```

---

### 授權條款

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
