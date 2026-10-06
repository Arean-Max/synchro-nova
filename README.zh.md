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

### 安装

从 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) 下载最新版本：

- **便携版 (`Synchro-Nova-*-Portable.zip`)** — 解压即可运行 `synchro.exe`，无需安装。
- **安装包 (`Synchro.Nova_*_x64-setup.exe`)** — 标准 Windows 安装程序，带快捷方式与卸载功能。

---

### 常见问题 (FAQ)

#### 这是什么工具？
专为竞技游戏打造的轻量工具。让画面色彩更饱满、提升目标辨识度并消除 Windows 系统冗余延迟。启动游戏时自动切换配置，退出游戏自动恢复。

#### 会被反作弊封号吗？
不会。Synchro Nova 不读写游戏内存、不注入 DLL、不创建屏幕覆盖层。它仅通过标准 Windows WinAPI 调节色彩，与 NVIDIA/AMD 显卡控制面板完全一致，对 VAC、Faceit、Vanguard、EAC 和 BattlEye 绝对安全。

#### 相比 VibranceGUI 或显卡驱动面板有何优势？
基于 Rust + Tauri 开发：秒级启动，在系统托盘仅占用约 4 MB 内存和 0% CPU。支持游戏专属自动配置切换与安全的系统延迟优化。

#### 系统优化会损坏 Windows 吗？
不会。仅应用经过验证的响应设置。在任何修改前均会自动创建 Windows 系统还原点和注册表备份，可随时一键还原。

---

### 源码构建

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 以开发模式运行
npm run build     # 构建发布安装包
```

---

### 开源协议

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
