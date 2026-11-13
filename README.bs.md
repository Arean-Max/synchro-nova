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

### Šta radi Synchro Nova?

Ukratko: to je lagani, besplatni uslužni program koji povećava živost boja i kontrast u igrama, pomažući neprijateljima da se istaknu u mračnom okruženju.

- **Automatski profili po igrici**: Postavite svoj omiljeni nivo vibracije za svoju igru ​​jednom. Kada pokrenete CS2, Valorant ili Apex, boje se automatski pojavljuju. Kada pritisnete alt-tab ili zatvorite, boje vašeg ekrana se odmah vraćaju u normalu tako da vas oči ne bole na radnoj površini ili u vašem pretraživaču.
- **Latencija i odziv**: Uključuje testirana, sigurna podešavanja Windows odziva (rezolucija tajmera, neobrađeni unos mišem, rukovanje mrežnim paketima) kako bi unos bio brz.
- **Zero Resource Drag**: Cijela aplikacija je manja od 5 MB, otvara se za milisekunde i nalazi se u vašoj traci koristeći samo ~2 MB RAM-a.

---

### Kako preuzeti i pokrenuti (Vodič za početnike)

Ako prvi put posjećujete GitHub, evo šta trebate učiniti:

1. Idite do [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) stranicu (ili kliknite na "Izdanja" na desnoj bočnoj traci).
2. Pod najnovijim izdanjem preuzmite:
   - **`synchro.exe`** — **prenosiva verzija (preporučuje se)**. Samo ga preuzmite u bilo koju mapu i dvaput kliknite za pokretanje. Nije potrebno podešavanje.
   - Or **`Synchro.Nova_*_x64-setup.exe`** — standardni Windows instalater ako više volite prečice na radnoj površini i start meniju.
3. **Ako Windows prikaže plavi skočni prozor ("Windows je zaštitio vaš računar" / SmartScreen)**:
   - Ovo se dešava sa svim besplatnim alatima otvorenog koda koji ne kupuju skupi sertifikat preduzeća od 500 USD godišnje od Microsofta.
   - Kliknite **"Više informacija"**, zatim kliknite **"Ipak trči"**.
   - Cijeli izvorni kod je 100% otvoren upravo ovdje na GitHubu — bez advera, bez telemetrije, bez gluposti.

---

### Brzi početak (1-minutna postavka)

1. Pokreni **Synchro Nova**.
2. Idi na **"Boja"** karticu i odaberite nivo vibrance (npr. `150%` ili `200%`).
3. Povežite ga sa svojom igrom (aplikacija automatski detektuje pokrenute igre ili možete odabrati `.exe` ručno).
4. To je to! Smanjite na tacnu i igrajte.

---

### FAQ

#### Da li ću biti zabranjen u CS2, Valorantu, Rustu ili Faceitu?
**100% Ne. Rizik zabrane je nula.**  
Synchro Nova **ne dodiruje memoriju igre, ne ubacuje DLL-ove u procese i ne crta prekrivače igre**. Podešava gamu desktopa samo preko standardnih Windows API-ja (DWM i GPU drajvera) — baš kao što prevlačite klizač Digital Vibrance na NVIDIA ili AMD kontrolnoj tabli. Anti-cheat motori (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) ne označavaju promjene boje radne površine.

#### Zašto jednostavno ne koristite NVIDIA ili AMD Control Panel?
GPU kontrolne ploče ne mijenjaju automatski boje kada pokrenete ili zatvorite igru. Ako tu pojačate živost, cijela Windows radna površina i pretraživač će ostati prezasićeni. Synchro to automatski rješava u hodu: igra radi = bogate boje; igra zatvorena = čiste boje radne površine.

#### Hoće li sistemska podešavanja upropastiti moj Windows?
Ne. Sva podešavanja koriste standardne, dobro dokumentovane Windows parametre. Prije izmjene bilo čega, Synchro Nova kreira Windows Restore Tačku i rezervnu kopiju modificiranih ključeva registratora. Sve možete vratiti na Windows zadane postavke bilo kada klikom **"Povratne promjene"**.

---

### Dokumentacija i duboko ronjenje

Da bi ova stranica bila čista i laka za čitanje, opsežna tehnička dokumentacija nalazi se na našem namjenskom wikiju:
- 📖 [Potpuna interaktivna dokumentacija na DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Dijagram arhitekture na GitDiagramu](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Kompletna referenca za podešavanja i registar (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Napravi iz izvora (za programere)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Saradnici

- [sotoykor-source](https://github.com/sotoykor-source) — Originalni kreator i uzvodni autor
- [Arean-Max](https://github.com/Arean-Max) — Razvoj i održavanje

---

### Licenca

Distribuirano pod [MIT License](LICENSE).
