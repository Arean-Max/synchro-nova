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

### シンクロノヴァって何をするの？

つまり、これは、ゲームの色の鮮やかさとコントラストを高め、暗い環境でも敵を目立たせることができる軽量の無料ユーティリティです。

- **ゲームごとの自動プロファイル**: ゲームのお気に入りの振動レベルを一度設定します。 CS2、Valorant、または Apex を起動すると、色が自動的に開始されます。 Alt-Tab キーを押すか終了すると、表示色はすぐに通常に戻るため、デスクトップやブラウザーで目が痛くなることはありません。
- **レイテンシと応答性**: 入力を軽快にするための、テスト済みの安全な Windows 応答性の調整 (タイマー解像度、生のマウス入力、ネットワーク パケット処理) が含まれています。
- **ゼロリソースドラッグ**: アプリ全体のサイズは 5 MB 未満で、数ミリ秒で開き、トレイに常駐するときの RAM の使用量はわずか約 2 MB です。

---

### ダウンロードと実行方法 (初心者向けガイド)

初めて GitHub にアクセスする場合は、次の手順を実行してください。

1. に向かってください。 [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) ページにアクセスします (または、右側のサイドバーの「リリース」をクリックします)。
2. 最新リリースでは、以下をダウンロードします。
   - **`synchro.exe`** — **ポータブル版 (推奨)**. 任意のフォルダーにダウンロードし、ダブルクリックして実行するだけです。セットアップは必要ありません。
   - または **`Synchro.Nova_*_x64-setup.exe`** — デスクトップとスタート メニューのショートカットを使用する場合は、標準の Windows インストーラーを使用します。
3. **Windows に青いポップアップが表示される場合 (「Windows が PC を保護しました」/SmartScreen)**:
   - これは、Microsoft から年間 500 ドルの高価なエンタープライズ証明書を購入しないすべての無料のオープンソース ツールで発生します。
   - クリック **"詳細情報"**, 次にクリックします **"とにかく走ってください"**.
   - ソース コード全体は GitHub 上で 100% 公開されており、アドウェアもテレメトリもナンセンスもありません。

---

### クイックスタート (1 分間のセットアップ)

1. 打ち上げ **Synchro Nova**.
2. に行きます **"色"** タブをクリックして、活気のレベルを選択します (例: `150%` または `200%`).
3. ゲームにリンクします (アプリは実行中のゲームを自動検出します。または、 `.exe` 手動で）。
4. それでおしまい！トレイに最小化して再生します。

---

### よくある質問

#### CS2、Valorant、Rust、または Faceit で禁止されますか?
**100% いいえ。禁止のリスクはゼロです。**  
Synchro Nova **ゲーム メモリにアクセスせず、プロセスに DLL を挿入せず、ゲーム オーバーレイを描画しません**. NVIDIA または AMD コントロール パネルで Digital Vibrance スライダーをドラッグするのとまったく同じように、標準の Windows API (DWM および GPU ドライバー) を介してデスクトップ ガンマを調整するだけです。アンチチート エンジン (VAC、Faceit、Vanguard、Easy Anti-Cheat、BattlEye) はデスクトップの色の変更にフラグを立てません。

#### NVIDIA または AMD コントロール パネルを使用しないのはなぜでしょうか?
GPU コントロール パネルは、ゲームを起動または終了するときに色を自動的に切り替えません。そこで活気を高めると、Windows デスクトップとブラウザ全体が過飽和状態になります。 Synchro はこれをオンザフライで自動的に処理します。ゲームの実行 = 豊富な色。ゲームが終了 = デスクトップの色がきれいになります。

#### システムの調整により Windows が混乱することはありますか?
いいえ。すべての調整では、十分に文書化された標準の Windows パラメータが使用されます。何かを変更する前に、Synchro Nova は Windows 復元ポイントと変更されたレジストリ キーのバックアップを作成します。をクリックすると、いつでもすべてを Windows のデフォルトに戻すことができます。 **"変更のロールバック"**.

---

### ドキュメントと詳細情報

このページをクリーンで読みやすく保つために、広範な技術文書が専用の Wiki にあります。
- 📖 [DeepWiki の完全なインタラクティブ ドキュメント](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram のアーキテクチャ図](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完全な調整とレジストリのリファレンス (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### ソースからビルドする (開発者向け)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### 貢献者

- [sotoykor-source](https://github.com/sotoykor-source) — オリジナルの作成者と上流の作成者
- [Arean-Max](https://github.com/Arean-Max) — 開発と保守

---

### ライセンス

のもとで配布されています [MIT License](LICENSE).
