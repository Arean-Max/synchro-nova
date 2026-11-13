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

### 为什么我建造这个

每次启动 CS2 或 Apex 时，我都必须打开 NVIDIA 控制面板，调高数字振动以发现黑暗角落中的敌人，然后在关闭游戏后手动将其调低，这样我的桌面和浏览器就不会灼伤我的视网膜。

Synchro 一劳永逸地解决了这个问题：
- **自动颜色切换**: 启动你的游戏，颜色开始生效。Alt-Tab 或关闭游戏，你的桌面颜色立即恢复正常。
- **延迟清理**: 终止 Windows 鼠标加速、优化计时器分辨率并修剪网络数据包缓冲，使输入感觉即时。
- **零膨胀**: 下载量低于 5 MB，在几毫秒内启动，并使用约 2 MB 的 RAM 安静地存放在托盘中。

> [!TIP]
> **经过测试的流行游戏 Vibrance 预设：**
> - **CS2（尘埃 II、幻影、地狱）**：设置为“150–165%”。玩家模型不再融入墙壁和板条箱。
> - **Valorant**：大约“135-140%”就足够了。游戏已有敌人轮廓亮点；较高的值只会洗掉颜色。
> - **Apex Legends 和 Rust**：“160–170%”对于在灌木丛中和远距离发现剪影具有奇效。
> - **双显示器**：Vibrance 仅适用于运行游戏的主显示器。您的第二个屏幕与 Discord 或 Chrome 保持正常颜色。

---

### 如何下载和运行（初学者指南）

如果您是 GitHub 的新手，以下是整个过程：

1. 单击此页面右侧的 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases)。
2. 下载 **`synchro.exe`** — 它是便携式版本。将其保存在任何地方并双击启动，无需安装程序。 （或者如果您想要开始菜单快捷方式，请获取“Synchro.Nova_*_x64-setup.exe”）。
3. **如果 Windows 显示蓝色 SmartScreen 弹出窗口（“Windows 保护了您的电脑”）**:
   - 不用担心。 Windows 为任何不从 Microsoft 购买每年 500 美元的公司签名的免费独立程序显示了这一点。
   - 单击 **“更多信息”** ➔ **“仍然运行”**。
   - 该项目在这里是 100% 开源的。零遥测、零矿工、零广告。

---

### 快速入门：3 个简单步骤

1. 启动“synchro.exe”。
2. 在 **“颜色”** 选项卡下，选择您的振动级别（例如“150%”）。
3. 从列表中选择您的游戏（或指向其“.exe”文件）。
最小化到托盘并播放。您再也不需要手动调整它。

---

### 常问问题

#### 我会在 CS2、Valorant、Rust 或 Faceit 中被禁止吗？
**100% 不会。禁令风险为零。**  
Synchro Nova **不接触游戏 RAM，不将 DLL 注入进程，也不绘制覆盖层**. 它纯粹通过标准 Windows 显示 API 来调整伽玛值，就像拖动 NVIDIA 或 AMD 控制面板中的滑块一样。反作弊系统（VAC、Faceit、Vanguard、EAC、BattlEye）甚至不将其视为游戏交互。

#### 为什么要在 NVIDIA/AMD 控制面板上使用它？
GPU 控制面板无法自动切换每个游戏的设置。如果您在那里开启振动，您的整个桌面在浏览或观看视频时就会保持过度饱和。 Synchro 会动态自动处理此问题。

#### Windows 调整安全吗？
是的。我们不会触及核心操作系统的完整性，不会破坏 Windows Defender，也不会禁用您的页面文件。在进行任何设置之前，Synchro 会创建 Windows 还原点和注册表备份。如果您不喜欢调整，请单击 **“回滚更改”** 立即恢复所有内容。

---

### 文档和深入研究

有关深入的技术细节，请查看我们的专用资源：
- 📖 [DeepWiki 上的完整交互式文档](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram 上的架构图](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完整的调整和注册表参考 (TWEAKS_REFERENCE.zh.md)](docs/TWEAKS_REFERENCE.zh.md)

---

### 从源代码构建（针对开发人员）

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 开发模式
npm run build     # 发布构建
```

---

### 贡献者

- [sotoykor-source](https://github.com/sotoykor-source) — 原创者&上游作者
- [Arean-Max](https://github.com/Arean-Max) — 开发与维护

---

### 执照

[MIT](LICENSE) — 免费使用、修改和共享。
