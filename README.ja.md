<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">モニターの色彩調整と Windows の入力遅延最適化のための軽量デスクトップユーティリティ。</p>

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

### インストール

[**Releases**](https://github.com/Arean-Max/synchro-nova/releases) から最新ビルドをダウンロードしてください：

- **ポータブル版 (`Synchro-Nova-*-Portable.zip`)** — 解凍して `synchro.exe` を実行するだけ。インストール不要。
- **インストーラー (`Synchro.Nova_*_x64-setup.exe`)** — ショートカットとアンインストーラー付きの標準セットアップ。

---

### よくある質問 (FAQ)

#### これはどのようなツールですか？
対戦ゲーム向けの軽量ツールです。ゲーム内の色彩を鮮やかにし、敵の視認性を高め、Windows の不要な遅延を削減します。ゲーム起動時に自動で適用されます。

#### アンチチートでBANされますか？
いいえ、100% 安全です。Synchro Nova はゲームのメモリを読み取らず、DLL 注入やオーバーレイも使用しません。NVIDIA や AMD の公式パネルと同様に標準 Windows WinAPI 経由で調整するため、VAC、Faceit、Vanguard、EAC、BattlEye でも安全です。

#### VibranceGUI や GPU コントロールパネルより何が優れていますか？
Rust + Tauri で開発されており、瞬時に起動し、トレイ待機時の CPU 使用率は 0%、メモリ消費は約 4 MB のみです。ゲームごとの自動プロファイル切り替えや安全なレイテンシ調整を備えています。

#### Windows を壊す心配はありませんか？
ありません。変更を適用する前に、自動的に Windows の復元ポイントとレジストリのバックアップを作成するため、いつでもワンクリックで元に戻せます。

---

### ソースコードからのビルド

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 開発モードで実行
npm run build     # インストーラーをビルド
```

---

### ライセンス

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
