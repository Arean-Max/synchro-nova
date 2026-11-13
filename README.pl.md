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

### Dlaczego to zbudowałem

Za każdym razem, gdy uruchamiałem CS2 lub Apex, musiałem otwierać Panel sterowania NVIDIA, podkręcać Digital Vibrance, aby wykryć wrogów w ciemnych zakamarkach, a następnie ręcznie zmniejszać ją po zamknięciu gry, aby mój komputer stacjonarny i przeglądarka nie spaliły moich siatkówek.

Synchro rozwiązuje ten problem raz na zawsze:
- **Automatyczne przełączanie kolorów**: Uruchom grę, kolory zaczną działać. Naciśnij klawisz Alt lub zamknij grę, a kolory pulpitu natychmiast wrócą do normy.
- **Oczyszczanie opóźnień**: Eliminuje przyspieszenie myszy w systemie Windows, optymalizuje rozdzielczość timera i ogranicza buforowanie pakietów sieciowych, dzięki czemu wprowadzanie danych jest natychmiastowe.
- **Zero wzdęć**: Pobieranie poniżej 5 MB rozpoczyna się w milisekundach i spokojnie leży na pasku zadań, zużywając ~2 MB pamięci RAM.

> [!TIP]
> **Przetestowane ustawienia wibracji dla popularnych gier:**
> - **CS2 (Dust II, Mirage, Inferno)**: Ustaw na „150–165%”. Modele graczy przestają wtapiać się w ściany i skrzynie.
> - **Valorant**: Około 135–140% w zupełności wystarczy. W grze znajdują się już zarysy wrogów; wyższe wartości po prostu rozmywają kolory.
> - **Apex Legends i Rust**: „160–170%” działa cuda przy wykrywaniu sylwetek w krzakach i na dużych dystansach.
> - **Dwa monitory**: Wibracja dotyczy tylko głównego monitora, na którym działa gra. Twój drugi ekran w Discord lub Chrome pozostaje w normalnych kolorach.

---

### Jak pobrać i uruchomić (przewodnik dla początkujących)

Jeśli dopiero zaczynasz korzystać z GitHuba, oto cały proces:

1. Kliknij [**Wydanie**](https://github.com/Arean-Max/synchro-nova/releases) po prawej stronie tej strony.
2. Pobierz **`synchro.exe`** — to wersja przenośna. Zapisz go w dowolnym miejscu i kliknij dwukrotnie, aby uruchomić, nie jest wymagany instalator. (Lub pobierz `Synchro.Nova_*_x64-setup.exe`, jeśli chcesz mieć skróty do menu Start).
3. **Jeśli system Windows wyświetla niebieskie wyskakujące okienko SmartScreen („Windows chronił twój komputer”)**:
   - Nie martw się. Windows pokazuje to dla każdego darmowego programu niezależnego, który nie kupuje podpisu korporacyjnego od Microsoftu za 500 dolarów rocznie.
   - Kliknij **„Więcej informacji”** ➔ **„Uruchom mimo to”**.
   - Tutaj projekt jest w 100% open source. Zero telemetrii, zero górników, zero reklam.

---

### Szybki start: 3 proste kroki

1. Uruchom `synchro.exe`.
2. W zakładce **„Kolor”** wybierz poziom jaskrawości (np. „150%”).
3. Wybierz swoją grę z listy (lub wskaż jej plik `.exe`).
Minimalizuj do zasobnika i graj. Już nigdy nie będziesz musiał regulować tego ręcznie.

---

### Często zadawane pytania

#### Czy dostanę bana w CS2, Valorant, Rust lub Faceit?
**100% Nie. Ryzyko bana wynosi zero.**  
Synchro Nova **nie dotyka pamięci RAM gry, nie dodaje bibliotek DLL do procesów i nie rysuje nakładek**. Dostosowuje gamma wyłącznie za pomocą standardowych interfejsów API wyświetlania systemu Windows — dokładnie tak, jak przeciąganie suwaka w panelu sterowania NVIDIA lub AMD. Systemy zapobiegające oszustwom (VAC, Faceit, Vanguard, EAC, BattlEye) nawet nie postrzegają tego jako interakcji w grze.

#### Po co używać tego w panelu sterowania NVIDIA/AMD?
Panele sterowania GPU nie mogą automatycznie przełączać ustawień dla każdej gry. Jeśli podkręcisz tam wibracje, cały pulpit pozostanie przesycony podczas przeglądania lub oglądania filmów. Synchro obsługuje to automatycznie na bieżąco.

#### Czy poprawki systemu Windows są bezpieczne?
Tak. Nie naruszamy integralności podstawowego systemu operacyjnego, nie psujemy programu Windows Defender i nie wyłączamy pliku stronicowania. Przed dotknięciem dowolnego ustawienia Synchro tworzy punkt przywracania systemu Windows i kopię zapasową rejestru. Jeśli nie podoba Ci się zmiana, kliknięcie **„Przywróć zmiany”** natychmiast przywraca wszystko.

---

### Dokumentacja i głębokie nurkowanie

Aby uzyskać szczegółowe informacje techniczne, sprawdź nasze dedykowane zasoby:
- 📖 [Pełna interaktywna dokumentacja na DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagram architektury na GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Kompletne poprawki i informacje o rejestrze (TWEAKS_REFERENCE.pl.md)](docs/TWEAKS_REFERENCE.pl.md)

---

### Kompiluj ze źródła (dla programistów)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # tryb rozwoju
npm run build     # wydanie kompilacji
```

---

### Współautorzy

- [sotoykor-source](https://github.com/sotoykor-source) — Oryginalny twórca i autor źródłowy
- [Arean-Max](https://github.com/Arean-Max) — Rozwój i utrzymanie

---

### Licencja

[MIT](LICENSE) — swobodnie używać, modyfikować i udostępniać.
