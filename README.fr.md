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

### Que fait Synchro Nova ?

En bref : il s'agit d'un utilitaire léger et gratuit qui améliore l'éclat et le contraste des couleurs dans les jeux, aidant ainsi les ennemis à se démarquer des environnements sombres.

- **Profils automatiques par jeu**: Définissez une fois votre niveau de vibrance préféré pour votre jeu. Lorsque vous lancez CS2, Valorant ou Apex, les couleurs apparaissent automatiquement. Lorsque vous alt-tab ou quittez, les couleurs de votre affichage reviennent immédiatement à la normale afin que vos yeux ne vous fassent pas mal sur le bureau ou dans votre navigateur.
- **Latence et réactivité**: Comprend des ajustements de réactivité Windows testés et sûrs (résolution de la minuterie, saisie brute de la souris, gestion des paquets réseau) pour rendre la saisie plus rapide.
- **Glissement de ressources nul**: L'application entière fait moins de 5 Mo, s'ouvre en millisecondes et se trouve dans votre barre d'état en utilisant seulement ~ 2 Mo de RAM.

---

### Comment télécharger et exécuter (Guide du débutant)

Si vous visitez GitHub pour la première fois, voici exactement ce qu'il faut faire :

1. Rendez-vous au [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) page (ou cliquez sur « Communications » dans la barre latérale droite).
2. Sous la dernière version, téléchargez :
   - **`synchro.exe`** — **version portable (recommandée)**. Téléchargez-le simplement dans n'importe quel dossier et double-cliquez pour l'exécuter. Aucune configuration nécessaire.
   - Ou **`Synchro.Nova_*_x64-setup.exe`** — programme d'installation Windows standard si vous préférez les raccourcis du bureau et du menu Démarrer.
3. **Si Windows affiche une fenêtre contextuelle bleue ("Windows a protégé votre PC" / SmartScreen)**:
   - Cela se produit avec tous les outils open source gratuits qui n'achètent pas un certificat d'entreprise coûteux de 500 $/an auprès de Microsoft.
   - Cliquez **"Plus d'informations"**, puis cliquez **"Courez quand même"**.
   - L'intégralité du code source est 100 % ouvert ici même sur GitHub : pas de logiciel publicitaire, pas de télémétrie, pas de bêtises.

---

### Démarrage rapide (installation en 1 minute)

1. Lancement **Synchro Nova**.
2. Allez au **"Couleur"** et choisissez votre niveau de vibrance (par ex. `150%` ou `200%`).
3. Liez-le à votre jeu (l'application détecte automatiquement les jeux en cours, ou vous pouvez choisir le `.exe` manuellement).
4. C'est ça! Réduisez-le dans le plateau et jouez.

---

### FAQ

#### Vais-je être banni dans CS2, Valorant, Rust ou Faceit ?
**100% Non. Le risque d’interdiction est nul.**  
Synchro Nova **ne touche pas à la mémoire du jeu, n'injecte pas de DLL dans les processus et ne dessine pas de superpositions de jeu**. Il ajuste uniquement le gamma du bureau via les API Windows standard (pilotes DWM et GPU) – exactement comme si vous faisiez glisser le curseur Digital Vibrance dans le panneau de configuration NVIDIA ou AMD. Les moteurs anti-triche (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) ne signalent pas les changements de couleur du bureau.

#### Pourquoi ne pas simplement utiliser le panneau de configuration NVIDIA ou AMD ?
Les panneaux de contrôle GPU ne changent pas automatiquement les couleurs lorsque vous lancez ou fermez un jeu. Si vous y augmentez la vibrance, l’ensemble de votre bureau et de votre navigateur Windows resteront sursaturés. Synchro gère cela automatiquement à la volée : jeu en cours = couleurs riches ; jeu fermé = couleurs propres du bureau.

#### Les modifications du système vont-elles gâcher mon Windows ?
Non. Tous les ajustements utilisent des paramètres Windows standard et bien documentés. Avant de modifier quoi que ce soit, Synchro Nova crée un point de restauration Windows et une sauvegarde des clés de registre modifiées. Vous pouvez tout rétablir aux paramètres par défaut de Windows à tout moment en cliquant sur **"Annulation des modifications"**.

---

### Documentation et analyse approfondie

Pour que cette page reste claire et facile à lire, une documentation technique complète se trouve sur notre wiki dédié :
- 📖 [Documentation interactive complète sur DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagramme d'architecture sur GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Modifications complètes et référence du registre (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Construire à partir des sources (pour les développeurs)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Contributeurs

- [sotoykor-source](https://github.com/sotoykor-source) — Créateur original et auteur en amont
- [Arean-Max](https://github.com/Arean-Max) — Développement & maintenance

---

### Licence

Distribué sous le [MIT License](LICENSE).
