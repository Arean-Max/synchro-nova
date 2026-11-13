<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Lagana alatka za pojačavanje boja monitora i optimizaciju odziva u sistemu Windows.</p>

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

### Zašto sam ovo napravio

Svaki put kada sam pokrenuo CS2 ili Apex, morao sam da otvorim NVIDIA Control Panel, da pokrenem Digital Vibrance da uočim neprijatelje u tamnim uglovima, a zatim da ga ručno okrenem nazad nakon zatvaranja igre kako mi desktop i pretraživač ne bi spalili mrežnjače.

Synchro ovo rješava jednom za svagda:
- **Automatsko prebacivanje boja**: Pokrenite svoju igru, boje se aktiviraju. Alt-Tab ili zatvorite igru, boje vaše radne površine se odmah vraćaju u normalu.
- **Čišćenje kašnjenja**: Ubija ubrzanje Windows miša, optimizuje rezoluciju tajmera i smanjuje baferovanje mrežnih paketa tako da se unos čini trenutnim.
- **Zero bloat**: Preuzimanje ispod 5 MB, počinje za milisekunde i tiho se nalazi u trai koristeći ~2 MB RAM-a.

> [!TIP]
> **Testirane postavke Vibrance za popularne igre:**
> - **CS2 (Dust II, Mirage, Inferno)**: Postavite na `150–165%`. Modeli igrača prestaju da se stapaju u zidove i sanduke.
> - **Valorant**: Oko `135–140%` je dovoljno. Igra već ima istaknute obrise neprijatelja; veće vrijednosti samo ispiraju boje.
> - **Apex Legends & Rust**: `160–170%` čini čuda za uočavanje silueta u žbunju i na velikim udaljenostima.
> - **Dvostruki monitori**: Vibrance se odnosi samo na primarni monitor koji pokreće vašu igru. Vaš drugi ekran sa Discordom ili Chromeom ostaje u normalnim bojama.

---

### Kako preuzeti i pokrenuti (Vodič za početnike)

Ako ste novi u GitHubu, evo cijelog procesa:

1. Kliknite na [**Izdanja**](https://github.com/Arean-Max/synchro-nova/releases) na desnoj strani ove stranice.
2. Preuzmite **`synchro.exe`** — to je prenosiva verzija. Sačuvajte ga bilo gdje i dvaput kliknite za pokretanje, nije potreban instalater. (Ili uzmite `Synchro.Nova_*_x64-setup.exe` ako želite prečice menija Start).
3. **Ako Windows prikaže plavi SmartScreen iskačući prozor ("Windows je zaštitio vaš računar")**:
   - Ne brini. Windows to pokazuje za bilo koji besplatni indie program koji ne kupuje korporativni potpis od 500 USD godišnje od Microsofta.
   - Kliknite **"Više informacija"** ➔ **"Pokreni svejedno"**.
   - Projekat je 100% otvorenog koda upravo ovdje. Nula telemetrija, nula rudara, nula oglasa.

---

### Brzi početak: 3 jednostavna koraka

1. Pokrenite `synchro.exe`.
2. Ispod kartice **"Boja"** odaberite nivo vibracije (npr. `150%`).
3. Odaberite svoju igru ​​sa liste (ili pokažite na njenu `.exe` datoteku).
Smanjite na ladicu i igrajte. Nikada više nećete morati ručno da ga podešavate.

---

### FAQ

#### Da li ću biti zabranjen u CS2, Valorantu, Rustu ili Faceitu?
**100% Ne. Rizik zabrane je nula.**  
Synchro Nova **ne dodiruje RAM igre, ne ubacuje DLL-ove u procese i ne crta preklapanja**. Podešava gamu isključivo putem standardnih Windows API-ja za ekrane — baš kao što prevlačite klizač na NVIDIA ili AMD kontrolnoj tabli. Sistemi protiv varanja (VAC, Faceit, Vanguard, EAC, BattlEye) to čak i ne vide kao interakciju u igri.

#### Zašto ovo koristiti preko NVIDIA / AMD kontrolne table?
GPU kontrolne ploče ne mogu automatski mijenjati postavke po igrici. Ako tamo aktivirate vibraciju, cijela radna površina ostaje prezasićena dok pretražujete ili gledate video zapise. Synchro to automatski rješava u hodu.

#### Da li su Windows podešavanja bezbedna?
Da. Ne diramo integritet osnovnog OS-a, ne razbijamo Windows Defender i ne onemogućavamo vašu datoteku stranice. Pre nego što dodirne bilo koju postavku, Synchro kreira Windows tačku vraćanja i rezervnu kopiju registratora. Ako vam se ne sviđa podešavanje, klikom na **"Vrati promjene"** sve se odmah vraća.

---

### Dokumentacija i duboko ronjenje

Za detaljne tehničke detalje pogledajte naše namjenske resurse:
- 📖 [Potpuna interaktivna dokumentacija na DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Dijagram arhitekture na GitDiagramu](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Kompletna referenca za podešavanja i registar (TWEAKS_REFERENCE.bs.md)](docs/TWEAKS_REFERENCE.bs.md)

---

### Napravi iz izvora (za programere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # način razvoja
npm run build     # izdanje build
```

---

### Saradnici

- [sotoykor-source](https://github.com/sotoykor-source) — Originalni kreator i uzvodni autor
- [Arean-Max](https://github.com/Arean-Max) — Razvoj i održavanje

---

### Licenca

[MIT](LICENSE) — besplatno korištenje, modificiranje i dijeljenje.
