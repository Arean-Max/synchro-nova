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

### Pourquoi j'ai construit ça

Chaque fois que je lançais CS2 ou Apex, je devais ouvrir le panneau de configuration NVIDIA, augmenter Digital Vibrance pour repérer les ennemis dans les coins sombres, puis le rappeler manuellement après la fermeture du jeu afin que mon bureau et mon navigateur ne me brûlent pas la rétine.

Synchro résout ce problème une fois pour toutes :
- **Changement de couleur automatique**: Lancez votre jeu, les couleurs entrent en jeu. Alt-Tab ou fermez le jeu, les couleurs de votre bureau reviennent immédiatement à la normale.
- **Nettoyage de la latence**: Supprime l'accélération de la souris Windows, optimise la résolution de la minuterie et réduit la mise en mémoire tampon des paquets réseau pour que la saisie soit instantanée.
- **Zéro ballonnement**: En moins de 5 Mo de téléchargement, démarre en millisecondes et reste silencieusement dans le bac en utilisant environ 2 Mo de RAM.

> [!TIP]
> **Préréglages Vibrance testés pour les jeux populaires :**
> - **CS2 (Dust II, Mirage, Inferno)** : réglé sur « 150–165 % ». Les modèles de joueurs ne se fondent plus dans les murs et les caisses.
> - **Valorant** : Environ « 135 à 140 % », c'est suffisant. Le jeu présente déjà les points forts des contours des ennemis ; des valeurs plus élevées ne font que décolorer les couleurs.
> - **Apex Legends & Rust** : « 160-170 % » fait des merveilles pour repérer des silhouettes dans les buissons et sur de longues distances.
> - **Deux moniteurs** : Vibrance s'applique uniquement au moniteur principal exécutant votre jeu. Votre deuxième écran avec Discord ou Chrome reste aux couleurs normales.

---

### Comment télécharger et exécuter (Guide du débutant)

Si vous êtes nouveau sur GitHub, voici l'ensemble du processus :

1. Cliquez sur [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) sur le côté droit de cette page.
2. Téléchargez **`synchro.exe`** — c'est la version portable. Enregistrez-le n'importe où et double-cliquez pour le lancer, aucun installateur n'est requis. (Ou récupérez `Synchro.Nova_*_x64-setup.exe` si vous voulez des raccourcis dans le menu Démarrer).
3. **Si Windows affiche une fenêtre contextuelle SmartScreen bleue (« Windows a protégé votre PC »)**:
   - Ne t'inquiète pas. Windows montre cela pour tout programme indépendant gratuit qui n'achète pas une signature d'entreprise de 500 $/an auprès de Microsoft.
   - Cliquez sur **"Plus d'infos"** ➔ **"Exécuter quand même"**.
   - Le projet est 100% open source ici. Zéro télémétrie, zéro mineur, zéro publicité.

---

### Démarrage rapide : 3 étapes simples

1. Lancez `synchro.exe`.
2. Sous l'onglet **"Couleur"**, choisissez votre niveau de vibrance (par exemple « 150 % »).
3. Sélectionnez votre jeu dans la liste (ou pointez sur son fichier « .exe »).
Réduire dans le plateau et jouer. Vous n'aurez plus jamais besoin de l'ajuster manuellement.

---

### FAQ

#### Vais-je être banni dans CS2, Valorant, Rust ou Faceit ?
**100% Non. Le risque d’interdiction est nul.**  
Synchro Nova **ne touche pas à la RAM du jeu, n'injecte pas de DLL dans les processus et ne dessine pas de superpositions**. Il ajuste le gamma uniquement via les API d'affichage Windows standard, exactement comme si vous faisiez glisser le curseur dans le panneau de configuration NVIDIA ou AMD. Les systèmes anti-triche (VAC, Faceit, Vanguard, EAC, BattlEye) ne le voient même pas comme une interaction de jeu.

#### Pourquoi l'utiliser sur le panneau de configuration NVIDIA/AMD ?
Les panneaux de contrôle GPU ne peuvent pas basculer automatiquement les paramètres par jeu. Si vous activez la vibrance à cet endroit, l'ensemble de votre bureau reste sursaturé lorsque vous naviguez ou regardez des vidéos. Synchro gère cela automatiquement à la volée.

#### Les ajustements de Windows sont-ils sûrs ?
Oui. Nous ne touchons pas à l’intégrité principale du système d’exploitation, nous ne cassons pas Windows Defender et nous ne désactivons pas votre fichier d’échange. Avant de toucher à un paramètre, Synchro crée un point de restauration Windows et une sauvegarde de registre. Si vous n'aimez pas un ajustement, cliquez sur **"Annuler les modifications"** pour tout rétablir immédiatement.

---

### Documentation et analyse approfondie

Pour des détails techniques détaillés, consultez nos ressources dédiées :
- 📖 [Documentation interactive complète sur DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagramme d'architecture sur GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Modifications complètes et référence du registre (TWEAKS_REFERENCE.fr.md)](docs/TWEAKS_REFERENCE.fr.md)

---

### Construire à partir des sources (pour les développeurs)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # mode de développement
npm run build     # version de version
```

---

### Contributeurs

- [sotoykor-source](https://github.com/sotoykor-source) — Créateur original et auteur en amont
- [Arean-Max](https://github.com/Arean-Max) — Développement & maintenance

---

### Licence

[MIT](LICENSE) — libre d'utilisation, de modification et de partage.
