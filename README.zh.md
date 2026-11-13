<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">轻量级桌面工具，用于显示器色彩饱和度增强与 Windows 延迟优化。</p>

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

### 同步新星有什么作用？

简而言之：它是一款轻量级的免费实用程序，可增强游戏中的色彩活力和对比度，帮助敌人在黑暗环境中脱颖而出。

- **自动每场比赛配置文件**: 为您的游戏设置一次您最喜欢的振动级别。当您启动 CS2、Valorant 或 Apex 时，颜色会自动生效。当您按 alt-tab 或退出时，显示颜色会立即恢复正常，这样您的眼睛就不会在桌面或浏览器中受到伤害。
- **延迟和响应能力**: 包括经过测试的、安全的 Windows 响应能力调整（计时器分辨率、原始鼠标输入、网络数据包处理），使输入感觉敏捷。
- **零资源拖累**: 整个应用程序不到 5 MB，可在几毫秒内打开，并且仅使用约 2 MB 的 RAM 即可放置在您的托盘中。

---

### 如何下载和运行（初学者指南）

如果您是第一次访问 GitHub，请执行以下操作：

1. 前往 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) 页面（或单击右侧边栏上的“发布”）。
2. 在最新版本下，下载：
   - **`synchro.exe`** — **便携版（推荐）**. 只需将其下载到任意文件夹并双击即可运行。无需设置。
   - 或者 **`Synchro.Nova_*_x64-setup.exe`** — 如果您喜欢桌面和开始菜单快捷方式，则可以使用标准 Windows 安装程序。
3. **如果 Windows 显示蓝色弹出窗口（“Windows 保护了您的电脑”/SmartScreen）**:
   - 所有免费的开源工具都会发生这种情况，而无需从 Microsoft 购买昂贵的每年 500 美元的企业证书。
   - 点击 **"更多信息"**, 然后单击 **"无论如何都要跑"**.
   - 整个源代码在 GitHub 上 100% 开放——没有广告软件，没有遥测，没有废话。

---

### 快速启动（1 分钟设置）

1. 发射 **Synchro Nova**.
2. 前往 **"颜色"** 选项卡并选择您的振动级别（例如 `150%` 或者 `200%`).
3. 将其链接到您的游戏（应用程序会自动检测正在运行的游戏，或者您可以选择 `.exe` 手动）。
4. 就是这样！最小化到托盘并播放。

---

### 常问问题

#### 我会在 CS2、Valorant、Rust 或 Faceit 中被禁止吗？
**100% 不会。禁令风险为零。**  
Synchro Nova **不接触游戏内存，不将 DLL 注入进程，也不绘制游戏覆盖图**. 它仅通过标准 Windows API（DWM 和 GPU 驱动程序）调整桌面伽玛 — 就像拖动 NVIDIA 或 AMD 控制面板中的 Digital Vibrance 滑块一样。反作弊引擎（VAC、Faceit、Vanguard、Easy Anti-Cheat、BattlEye）不会标记桌面颜色变化。

#### 为什么不直接使用 NVIDIA 或 AMD 控制面板？
当您启动或关闭游戏时，GPU 控制面板不会自动切换颜色。如果您在那里增加活力，您的整个 Windows 桌面和浏览器将保持过度饱和状态。 Synchro 会动态自动处理此问题：游戏运行 = 丰富的色彩；游戏关闭=干净的桌面颜色。

#### 系统调整会扰乱我的 Windows 吗？
不会。所有调整都使用标准的、有据可查的 Windows 参数。在修改任何内容之前，Synchro Nova 会创建一个 Windows 还原点和修改后的注册表项的备份。您可以随时通过单击将所有内容恢复为 Windows 默认设置 **"回滚更改"**.

---

### 文档和深入研究

为了保持此页面干净且易于阅读，我们的专用 wiki 上有大量的技术文档：
- 📖 [DeepWiki 上的完整交互式文档](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram 上的架构图](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完整的调整和注册表参考 (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### 从源代码构建（针对开发人员）

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### 贡献者

- [sotoykor-source](https://github.com/sotoykor-source) — 原创者&上游作者
- [Arean-Max](https://github.com/Arean-Max) — 开发与维护

---

### 执照

分布在 [MIT License](LICENSE).
