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

### これを作った理由

CS2 や Apex を起動するたびに、NVIDIA コントロール パネルを開き、暗い隅の敵を見つけるために Digital Vibrance を上げ、デスクトップとブラウザが網膜を焼き付けないように、ゲームを閉じた後に手動でダイヤルを下げなければなりませんでした。

Synchro はこれを完全に解決します。
- **自動カラー切り替え**: ゲームを起動すると、色が表示されます。Alt-Tab を押すか、ゲームを閉じると、デスクトップの色はすぐに通常の色に戻ります。
- **レイテンシのクリーンアップ**: Windows マウスのアクセラレーションを停止し、タイマー解像度を最適化し、ネットワーク パケット バッファリングをトリミングして、入力が瞬時に感じられるようにします。
- **膨張ゼロ**: ダウンロードは 5 MB 未満で、ミリ秒以内に開始され、最大 2 MB の RAM を使用してトレイに静かに配置されます。

> [!TIP]
> **人気のゲーム向けにテスト済みの Vibrance プリセット:**
> - **CS2 (ダスト II、ミラージュ、インフェルノ)**: `150–165%` に設定します。プレイヤーモデルが壁や木箱に溶け込まなくなります。
> - **Valorant**:「135 ～ 140%」程度で十分です。ゲームにはすでに敵の輪郭のハイライトが含まれています。値を大きくすると、色が洗い流されるだけです。
> - **Apex Legends & Rust**: 「160–170%」は、茂みや長距離のシルエットを見つけるのに驚異的な効果を発揮します。
> - **デュアル モニター**: 振動は、ゲームを実行しているプラ​​イマリ モニターにのみ適用されます。 Discord または Chrome の 2 番目の画面は通常の色のままです。

---

### ダウンロードと実行方法 (初心者向けガイド)

GitHub を初めて使用する場合は、プロセス全体を次に示します。

1. このページの右側にある [**リリース**](https://github.com/Arean-Max/synchro-nova/releases) をクリックします。
2. **`synchro.exe`** をダウンロードします。これはポータブル バージョンです。任意の場所に保存し、ダブルクリックして起動します。インストーラーは必要ありません。 (または、[スタート] メニューのショートカットが必要な場合は、`Synchro.Nova_*_x64-setup.exe` を取得します)。
3. **Windows に青い SmartScreen ポップアップ (「Windows が PC を保護しました」) が表示される場合**:
   - 心配しないで。 Windows では、Microsoft から年間 500 ドルの企業署名を購入しない無料のインディーズ プログラムに対してこれが表示されます。
   - **「詳細」** ➔ **「とにかく実行」** をクリックします。
   - このプロジェクトは 100% オープンソースです。テレメトリーゼロ、マイナーゼロ、広告ゼロ。

---

### クイックスタート: 3 つの簡単なステップ

1. 「synchro.exe」を起動します。
2. **「カラー」** タブで、鮮やかさのレベルを選択します (例: `150%`)。
3. リストからゲームを選択します (または、その `.exe` ファイルをポイントします)。
トレイに最小化してプレイします。手動で再度調整する必要はありません。

---

### よくある質問

#### CS2、Valorant、Rust、または Faceit で禁止されますか?
**100% いいえ。禁止のリスクはゼロです。**  
Synchro Nova **ゲーム RAM には触れず、DLL をプロセスに挿入せず、オーバーレイも描画しません**. NVIDIA または AMD コントロール パネルでスライダーをドラッグするのとまったく同じように、標準の Windows ディスプレイ API を通じてガンマを調整します。アンチチート システム (VAC、Faceit、Vanguard、EAC、BattlEye) は、これをゲームのインタラクションとはみなしません。

#### NVIDIA/AMD コントロール パネル上でこれを使用するのはなぜですか?
GPU コントロール パネルは、ゲームごとに設定を自動的に切り替えることはできません。そこでバイブレーションを大きくすると、ブラウジングまたはビデオの視聴中にデスクトップ全体が過飽和のままになります。 Synchro はこれをオンザフライで自動的に処理します。

#### Windows の調整は安全ですか?
はい。私たちはコア OS の整合性には触れませんし、Windows Defender を破壊したり、ページファイルを無効にしたりしません。設定を変更する前に、Synchro は Windows 復元ポイントとレジストリ バックアップを作成します。調整が気に入らない場合は、**「変更をロールバック」** をクリックすると、すべてがすぐに元に戻ります。

---

### ドキュメントと詳細情報

技術的な詳細については、専用のリソースを確認してください。
- 📖 [DeepWiki の完全なインタラクティブ ドキュメント](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram のアーキテクチャ図](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [完全な調整とレジストリのリファレンス (TWEAKS_REFERENCE.ja.md)](docs/TWEAKS_REFERENCE.ja.md)

---

### ソースからビルドする (開発者向け)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 開発モード
npm run build     # リリースビルド
```

---

### 貢献者

- [sotoykor-source](https://github.com/sotoykor-source) — オリジナルの作成者と上流の作成者
- [Arean-Max](https://github.com/Arean-Max) — 開発と保守

---

### ライセンス

[MIT](LICENSE) — 自由に使用、変更、共有できます。
