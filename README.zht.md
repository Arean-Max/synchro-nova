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

### 同步新星有什麼作用？

簡而言之：它是一款輕量級的免費實用程序，可增強遊戲中的色彩活力和對比度，幫助敵人在黑暗環境中脫穎而出。

- **自動每場比賽設定檔**: 為您的遊戲設定一次您最喜歡的振動等級。當您啟動 CS2、Valorant 或 Apex 時，顏色會自動生效。當您按 alt-tab 或退出時，顯示顏色會立即恢復正常，因此您的眼睛不會在桌面或瀏覽器中受到傷害。
- **延遲和響應能力**: 包括經過測試的、安全的 Windows 響應能力調整（計時器解析度、原始滑鼠輸入、網路封包處理），讓輸入感覺敏捷。
- **零資源拖累**: 整個應用程式不到 5 MB，可在幾毫秒內打開，並且僅使用約 2 MB 的 RAM 即可放置在您的托盤中。

---

### 如何下載和運行（初學者指南）

如果您是第一次造訪 GitHub，請執行以下操作：

1. 前往 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) 頁面（或按一下右側邊欄上的「發布」）。
2. 在最新版本下，下載：
   - **`synchro.exe`** — **隨身版（推薦）**. 只需將其下載到任何資料夾並雙擊即可運行。無需設定。
   - 或者 **`Synchro.Nova_*_x64-setup.exe`** — 如果您喜歡桌面和開始功能表捷徑，則可以使用標準 Windows 安裝程式。
3. **如果 Windows 顯示藍色彈出視窗（「Windows 保護了您的電腦」/SmartScreen）**:
   - 所有免費的開源工具都會發生這種情況，而無需從 Microsoft 購買昂貴的每年 500 美元的企業憑證。
   - 點選 **"更多資訊"**, 然後單擊 **"無論如何都要跑"**.
   - 整個原始碼在 GitHub 上 100% 開放——沒有廣告軟體，沒有遙測，沒有廢話。

---

### 快速啟動（1 分鐘設定）

1. 發射 **Synchro Nova**.
2. 前往 **"顏色"** 選項卡並選擇您的振動等級（例如 `150%` 或者 `200%`).
3. 將其連結到您的遊戲（應用程式會自動檢測正在運行的遊戲，或者您可以選擇 `.exe` 手動）。
4. 就是這樣！最小化到托盤並播放。

---

### 常問問題

#### 我會在 CS2、Valorant、Rust 或 Faceit 中被禁止嗎？
**100% 不會。禁令風險為零。**  
Synchro Nova **不接觸遊戲內存，不將 DLL 注入進程，也不繪製遊戲覆蓋圖**. 它僅透過標準 Windows API（DWM 和 GPU 驅動程式）調整桌面伽瑪 — 就像拖曳 NVIDIA 或 AMD 控制面板中的 Digital Vibrance 滑桿一樣。反作弊引擎（VAC、Faceit、Vanguard、Easy Anti-Cheat、BattlEye）不會標記桌面顏色變更。

#### 為什麼不直接使用 NVIDIA 或 AMD 控制面板？
當您啟動或關閉遊戲時，GPU 控制面板不會自動切換顏色。如果您在那裡增加活力，您的整個 Windows 桌面和瀏覽器將保持過度飽和狀態。 Synchro 會動態自動處理此問題：遊戲運行 = 豐富的色彩；遊戲關閉=乾淨的桌面顏色。

#### 系統調整會擾亂我的 Windows 嗎？
不會。所有調整都使用標準的、有據可查的 Windows 參數。在修改任何內容之前，Synchro Nova 會建立一個 Windows 還原點和修改後的登錄項目的備份。您可以隨時透過點擊將所有內容還原為 Windows 預設設定 **"復原更改"**.

---

### 文件和深入研究

為了保持此頁面乾淨且易於閱讀，我們的專用 wiki 上有大量的技術文件：
- 📖 [DeepWiki 上的完整互動式文檔](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram 上的架構圖](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完整的調整和註冊表參考 (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### 從原始碼建置（針對開發人員）

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### 貢獻者

- [sotoykor-source](https://github.com/sotoykor-source) — 原創者&上游作者
- [Arean-Max](https://github.com/Arean-Max) — 開發與維護

---

### 執照

分佈在 [MIT License](LICENSE).
