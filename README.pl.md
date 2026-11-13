<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Lekkie narzędzie do regulacji nasycenia kolorów monitora i optymalizacji opóźnień w systemie Windows.</p>

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

### Co robi SynchroNova?

W skrócie: jest to lekkie, bezpłatne narzędzie, które zwiększa intensywność kolorów i kontrast w grach, pomagając wrogom wyróżnić się na tle ciemnego otoczenia.

- **Automatyczne profile dla poszczególnych gier**: Ustaw raz swój ulubiony poziom wibracji dla swojej gry. Po uruchomieniu CS2, Valorant lub Apex kolory włączają się automatycznie. Kiedy naciśniesz Alt-Tab lub wyjdziesz, kolory wyświetlacza natychmiast wracają do normy, więc oczy nie bolą Cię na pulpicie lub w przeglądarce.
- **Opóźnienie i responsywność**: Zawiera przetestowane, bezpieczne poprawki w zakresie responsywności systemu Windows (rozdzielczość timera, surowe wprowadzanie myszy, obsługa pakietów sieciowych), aby wprowadzanie danych było szybkie.
- **Zerowe przeciąganie zasobów**: Cała aplikacja waży mniej niż 5 MB, otwiera się w ciągu milisekund i znajduje się w zasobniku, zużywając zaledwie ~2 MB pamięci RAM.

---

### Jak pobrać i uruchomić (przewodnik dla początkujących)

Jeśli odwiedzasz GitHub po raz pierwszy, oto dokładnie, co musisz zrobić:

1. Udaj się do [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) (lub kliknij „Wersje” na prawym pasku bocznym).
2. W ramach najnowszej wersji pobierz:
   - **`synchro.exe`** — **wersja przenośna (zalecana)**. Wystarczy pobrać go do dowolnego folderu i kliknąć dwukrotnie, aby uruchomić. Nie jest wymagana żadna konfiguracja.
   - Lub **`Synchro.Nova_*_x64-setup.exe`** — standardowy instalator Windows, jeśli wolisz skróty na pulpicie i w menu Start.
3. **Jeśli system Windows wyświetla niebieskie wyskakujące okienko („Windows chronił twój komputer” / SmartScreen)**:
   - Dzieje się tak w przypadku wszystkich bezpłatnych narzędzi typu open source, które nie wymagają zakupu drogiego certyfikatu korporacyjnego o wartości 500 USD rocznie od firmy Microsoft.
   - Trzask **"Więcej informacji"**, następnie kliknij **"Mimo wszystko biegnij"**.
   - Cały kod źródłowy jest w 100% otwarty tutaj, w GitHubie — bez oprogramowania reklamowego, bez telemetrii, bez bzdur.

---

### Szybki start (1-minutowa konfiguracja)

1. Początek **Synchro Nova**.
2. Idź do **"Kolor"** i wybierz poziom wibracji (np. `150%` Lub `200%`).
3. Połącz go ze swoją grą (aplikacja automatycznie wykrywa uruchomione gry lub możesz wybrać `.exe` ręcznie).
4. To wszystko! Minimalizuj do zasobnika i graj.

---

### Często zadawane pytania

#### Czy dostanę bana w CS2, Valorant, Rust lub Faceit?
**100% Nie. Ryzyko bana wynosi zero.**  
Synchro Nova **nie dotyka pamięci gry, nie dodaje bibliotek DLL do procesów i nie rysuje nakładek gier**. Dostosowuje gamma pulpitu wyłącznie za pomocą standardowych interfejsów API systemu Windows (sterowniki DWM i GPU) — dokładnie tak, jak przeciąganie suwaka Digital Vibrance w panelu sterowania NVIDIA lub AMD. Silniki zapobiegające oszustwom (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) nie flagują zmian kolorów pulpitu.

#### Dlaczego po prostu nie skorzystać z Panelu sterowania NVIDIA lub AMD?
Panele sterowania GPU nie przełączają automatycznie kolorów po uruchomieniu lub zamknięciu gry. Jeśli zwiększysz tam intensywność, cały pulpit systemu Windows i przeglądarka pozostaną przesycone. Synchro obsługuje to automatycznie na bieżąco: gra działa = bogate kolory; gra zamknięta = czyste kolory pulpitu.

#### Czy zmiany w systemie spowodują uszkodzenie mojego systemu Windows?
Nie. Wszystkie poprawki wykorzystują standardowe, dobrze udokumentowane parametry systemu Windows. Przed modyfikacją czegokolwiek Synchro Nova tworzy punkt przywracania systemu Windows i kopię zapasową zmodyfikowanych kluczy rejestru. W każdej chwili możesz przywrócić ustawienia domyślne systemu Windows, klikając **"Wycofanie zmian"**.

---

### Dokumentacja i głębokie nurkowanie

Aby ta strona była czysta i łatwa do odczytania, na naszej dedykowanej wiki znajduje się obszerna dokumentacja techniczna:
- 📖 [Pełna interaktywna dokumentacja na DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagram architektury na GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Kompletne poprawki i informacje o rejestrze (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Kompiluj ze źródła (dla programistów)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Współautorzy

- [sotoykor-source](https://github.com/sotoykor-source) — Oryginalny twórca i autor źródłowy
- [Arean-Max](https://github.com/Arean-Max) — Rozwój i utrzymanie

---

### Licencja

Rozpowszechniany w ramach [MIT License](LICENSE).
