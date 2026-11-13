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

### 為什麼我要建造這個

每次啟動 CS2 或 Apex 時，我都必須打開 NVIDIA 控制面板，調高數位振動以發現黑暗角落中的敵人，然後在關閉遊戲後手動將其調低，這樣我的桌面和瀏覽器就不會灼傷我的視網膜。

Synchro 一勞永逸地解決了這個問題：
- **自動顏色切換**: 啟動你的遊戲，顏色開始生效。 Alt-Tab 或關閉遊戲，你的桌面顏色立即恢復正常。
- **延遲清理**: 終止 Windows 滑鼠加速、最佳化計時器解析度並修剪網路封包緩衝，使輸入感覺即時。
- **零膨脹**: 下載量低於 5 MB，在幾毫秒內啟動，並使用約 2 MB 的 RAM 安靜地存放在托盤中。

> [!TIP]
> **經過測試的流行遊戲 Vibrance 預設：**
> - **CS2（塵埃 II、幻影、地獄）**：設定為「150–165%」。玩家模型不再融入牆壁和板條箱。
> - **Valorant**：大約「135-140%」就足夠了。遊戲已有敵人輪廓亮點；較高的數值只會洗掉顏色。
> - **Apex Legends 和 Rust**：「160–170%」對於在灌木叢中和遠距離發現剪影具有奇效。
> - **雙顯示器**：Vibrance 僅適用於運行遊戲的主顯示器。您的第二個螢幕與 Discord 或 Chrome 保持正常顏色。

---

### 如何下載和運行（初學者指南）

如果您是 GitHub 的新手，以下是整個過程：

1. 點擊此頁面右側的 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases)。
2. 下載 **`synchro.exe`** — 它是便攜式版本。將其保存在任何地方並雙擊啟動，無需安裝程式。 （或如果您想要開始功能表快捷方式，請取得“Synchro.Nova_*_x64-setup.exe”）。
3. **如果 Windows 顯示藍色 SmartScreen 彈出視窗（「Windows 保護了您的電腦」）**:
   - 不用擔心。 Windows 為任何不從 Microsoft 購買每年 500 美元的公司簽署的免費獨立程式顯示了這一點。
   - 按一下 **“更多資訊”** ➔ **“仍然運行”**。
   - 該專案在這裡是 100% 開源的。零遙測、零礦工、零廣告。

---

### 快速入門：3 個簡單步驟

1. 啟動“synchro.exe”。
2. 在 **「顏色」** 標籤下，選擇您的振動等級（例如「150%」）。
3. 從清單中選擇您的遊戲（或指向其“.exe”檔案）。
最小化到托盤並播放。您再也不需要手動調整它。

---

### 常問問題

#### 我會在 CS2、Valorant、Rust 或 Faceit 中被禁止嗎？
**100% 不會。禁令風險為零。**  
Synchro Nova **不接觸遊戲 RAM，不將 DLL 注入進程，也不繪製覆蓋層**. 它純粹透過標準 Windows 顯示 API 來調整伽瑪值，就像拖曳 NVIDIA 或 AMD 控制面板中的滑桿一樣。反作弊系統（VAC、Faceit、Vanguard、EAC、BattlEye）甚至不將其視為遊戲互動。

#### 為什麼要在 NVIDIA/AMD 控制面板上使用它？
GPU 控制面板無法自動切換每個遊戲的設定。如果您在那裡開啟振動，您的整個桌面在瀏覽或觀看影片時就會保持過度飽和。 Synchro 會動態自動處理此問題。

#### Windows 調整安全性嗎？
是的。我們不會觸及核心作業系統的完整性，不會破壞 Windows Defender，也不會停用您的頁面檔案。在進行任何設定之前，Synchro 會建立 Windows 還原點和登錄備份。如果您不喜歡調整，請按一下 **「回滾變更」** 立即恢復所有內容。

---

### 文件和深入研究

有關深入的技術細節，請查看我們的專用資源：
- 📖 [DeepWiki 上的完整互動式文檔](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram 上的架構圖](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完整的調整和註冊表參考 (TWEAKS_REFERENCE.zht.md)](docs/TWEAKS_REFERENCE.zht.md)

---

### 從原始碼建置（針對開發人員）

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 開發模式
npm run build     # 發布構建
```

---

### 貢獻者

- [sotoykor-source](https://github.com/sotoykor-source) — 原創者&上游作者
- [Arean-Max](https://github.com/Arean-Max) — 開發與維護

---

### 執照

[MIT](LICENSE) — 免費使用、修改和分享。
