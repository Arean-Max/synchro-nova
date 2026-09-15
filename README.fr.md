<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Utilitaire de bureau léger pour la vibrance des couleurs et l'optimisation de la latence Windows.</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
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

### Installation

Téléchargez la dernière version sur [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) :

- **Version portable (`Synchro-Nova-*-Portable.zip`)** — Décompressez et lancez `synchro.exe`. Aucune installation requise.
- **Installateur (`Synchro.Nova_*_x64-setup.exe`)** — Programme d'installation standard pour Windows.

---

### FAQ

#### Qu'est-ce que cet outil ?
Un utilitaire ultra-léger pour les joueurs. Il renforce les couleurs en jeu, améliore la visibilité des ennemis et réduit la latence Windows. Les profils basculent automatiquement au lancement des jeux.

#### Est-ce bannissable par les anticheats ?
Non. Synchro Nova n'accède pas à la mémoire du jeu, n'injecte aucune DLL et n'utilise pas d'overlay. L'affichage est ajusté via les WinAPI standards de Windows, exactement comme les panneaux NVIDIA ou AMD. 100% sûr pour VAC, Faceit, Vanguard, EAC et BattlEye.

#### Pourquoi est-ce meilleur que VibranceGUI ?
Développé en Rust + Tauri : démarrage instantané, 0% de CPU et seulement ~4 Mo de RAM en arrière-plan. Prend en charge les profils automatiques par jeu et des optimisations saines de latence.

#### Les réglages peuvent-ils endommager Windows ?
Non. Seuls des paramètres fiables sont appliqués. Un point de restauration Windows et une sauvegarde du registre sont créés automatiquement avant toute modification.

---

### Compiler depuis les sources

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # lancer en mode développement
npm run build     # compiler l'installateur
```

---

### Licence

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

