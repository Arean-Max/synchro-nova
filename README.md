[![](https://img.shields.io/badge/Language-🇬🇧_Switch_to_English-0284c7?style=for-the-badge)](README.en.md)

# Synchro Nova

<div align="center">

[![Telegram](https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white)](https://t.me/synchronova)
[![Releases](https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald)](https://github.com/Arean-Max/synchro-nova/releases)
[![License](https://img.shields.io/badge/License-MIT-white?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square)](https://microsoft.com/windows)

**Десктопная утилита для аппаратной цветокоррекции монитора, оптимизации задержки ввода и тонкой настройки Windows.**  
*Написана на Rust и Tauri v2. Потребляет ~2 МБ RAM в трее, использует 0% CPU в простое и работает без внедрения в память сторонних процессов.*

<br>

<img src="docs/screenshots/main-frame.png" alt="Synchro Nova Interface" width="880" style="border-radius: 10px; box-shadow: 0 8px 30px rgba(0,0,0,0.5);">

</div>

---

## О проекте

Synchro Nova создавалась как замена громоздким оверлеям и сомнительным утилитам, которые забивают оперативную память, внедряются в системные библиотеки и вызывают конфликты с античитами в играх. 

Приложение объединяет аппаратную регулировку сочности (Digital Vibrance), гамма-калибровку через LUT монитора, аудит драйверов и системные твики в одном легковесном решении без рекламы, телеметрии и фоновых служб.

---

## 🛡️ Безопасность и иммунитет к античитам (Anti-Cheat Safety)

Многие пользователи соревновательных игр (Valorant, CS2, Fortnite, Warzone, Apex Legends, Rainbow Six Siege) сталкивались с банами из-за сторонних программ, таких как **Tactical Vision**.

### Почему античиты блокировали Tactical Vision?
1. **Инъекция DLL в процесс игры**: Tactical Vision использовал инъекции динамических библиотек (`CreateRemoteThread`, `LoadLibrary`) непосредственно в адресное пространство процесса игры.
2. **Перехват графического конвейера (DirectX / Vulkan SwapChain Hooks)**: программа хукала системные вызовы `Present` / `Present1` в `dxgi.dll` и `d3d11.dll` для отрисовки поверх кадров игры. Античиты уровня ядра (Riot Vanguard, Easy Anti-Cheat, BattlEye, Valve Anti-Cheat, Ricochet) расценивают любые перехваты DirectX как читы (Wallhack, ESP, Shader Chams).
3. **Эмуляция ввода и симуляция клавиатуры/мыши**: перехват и симуляция событий через `keybd_event`, `SendInput` или хуки `SetWindowsHookEx`.
4. **Чтение и модификация чужой памяти**: вызовы `OpenProcess` с правами `PROCESS_VM_READ` и `PROCESS_VM_WRITE`.

### Почему Synchro Nova на 100% безопасна и исключает бан?
- **Абсолютная изоляция (Zero Process Injection)**: Synchro Nova работает исключительно в пользовательском режиме (User Mode) как отдельный изолированный процесс. Приложение не открывает дескрипторы игр и не обращается к чужой памяти (0 вызовов `PROCESS_VM_READ` или `PROCESS_VM_WRITE`).
- **Никаких хуков графики (Zero DirectX/Vulkan Hooks)**: В приложении нет ни единого хука к `dxgi.dll`, `d3d11.dll`, `d3d12.dll` или `vulkan-1.dll`. Synchro Nova не перехватывает игровой SwapChain.
- **Никакой симуляции ввода**: В кодовой базе Synchro Nova полностью отсутствуют вызовы синтетического ввода (`keybd_event`, `mouse_event`, `SendInput`) и глобальные оконные хуки.
- **Аппаратный LUT монитора и системный DWM**:
  - Цветовая насыщенность, баланс белого и контраст регулируются через официальный WinAPI интерфейс композитора Windows Desktop Window Manager (`MagSetFullscreenColorEffect` из `magnification.dll`).
  - Гамма-калибровка передается напрямую в видеокарту и LUT дисплея (`SetDeviceGammaRamp` из `gdi32.dll`).
  - Античиты ядра контролируют исключительно память самой игры и не вмешиваются в работу композитора рабочего стола Windows — точно так же, как они не банят за регулировку Digital Vibrance в официальной панели управления NVIDIA / AMD Software или за ночной режим Windows.
- **Встроенные политики безопасности Windows**: При старте Synchro Nova активирует системные политики DEP (Data Execution Prevention), блокировку DLL Hijacking (Safe Search Mode) и защиту целостности кучи.

---

## Основные возможности

### 1. Аппаратная цветокоррекция (Display Calibration)
- **Управление через Windows GDI & Magnification API**: сочность (vibrance), насыщенность, гамма, контраст и цветовой тон без задержки кадра и без инпут-лага.
- **Режим Black Holo**: интеллектуальная калибровка контраста и глубоких теней в соревновательных шутерах (CS2, Rust, Apex, Tarkov) без пересвета светлых участков.
- **Профили для игр**: возможность привязать индивидуальные параметры калибровки к конкретным играм.
- **Кастомизация акцента**: поддержка любых RGB цветов интерфейса с автоматическим расчетом читаемости текста и адаптивной контрастностью.

### 2. Системные твики и оптимизация отклика (System Tweaks)
- **Проверенные параметры**: оптимизация системного таймера, приоритеты MMCSS, полноэкранная оптимизация (FSO), отключение телеметрии и оптимизация сетевого стека TCP.
- **Точки восстановления Windows и бекапы реестра**: при создании резервной копии приложение автоматически создаёт точку восстановления системы Windows (`SRSetRestorePointW` / `Checkpoint-Computer`) и снимок реестра (`.reg`), гарантируя 100% возможность возврата к исходным настройкам.
- **Прозрачность**: каждая команда и ключ реестра документированы в интерфейсе и в [справочнике по твикам](docs/TWEAKS_REFERENCE.md).

### 3. Анализ драйверов и ПК (Hardware & Drivers)
- Определение видеокарты, текущей версии видеодрайвера и даты его выпуска.
- Проверка актуальности драйверов NVIDIA, AMD, Intel с официальными ссылками на страницы загрузки вендоров.
- Мониторинг загрузки процессора и оперативной памяти через нативный Windows PDH (Performance Data Helper).

---

## Производительность и ресурсы

| Показатель | Synchro Nova v2.1 | Типичный софт на Electron |
|---|:---:|:---:|
| **Оперативная память в трее** | **~1.5 – 3 МБ** | 120 – 350 МБ |
| **Нагрузка на CPU в фоне** | **0.0% (0 прерываний таймера)** | 0.5 – 2.5% |
| **Время холодного старта** | **< 0.3 сек** | 2.5 – 6.0 сек |
| **Внешний сетевой трафик** | **0 байт (телеметрия Chromium вырезана)** | Постоянные аналитические запросы |

---

## Скачать и установить

Свежие сборки всегда доступны на странице [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Портативная версия (`Synchro-Nova-2.1.0-Portable.zip`)** — автономный бинарник `synchro.exe`, запускается сразу из любой папки или с флешки, не требует установки и не оставляет следов в системе.
- **Установщик (`Synchro.Nova_2.1.0_x64-setup.exe`)** — классический установщик с ярлыком на рабочем столе, интеграцией в системный поиск Windows («synchro») и чистым деинсталлятором.

### Проверка контрольной суммы (SHA-256)
Для проверки подлинности скачанного файла откройте PowerShell в папке с файлом:
```powershell
Get-FileHash .\synchro.exe -Algorithm SHA256
```

---

## Сборка из исходников

### Требования
- [Node.js](https://nodejs.org/) (версия 18+)
- [Rust](https://www.rust-lang.org/) (stable `x86_64-pc-windows-gnu` или `x86_64-pc-windows-msvc`)
- [Tauri CLI](https://tauri.app/)

### Инструкция по сборке
```bash
# 1. Клонируйте репозиторий
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova

# 2. Установите зависимости
npm install

# 3. Запуск в режиме разработки
npm run dev

# 4. Сборка портативной версии
npm run build:portable

# 5. Сборка релизного инсталлятора
npm run build
```

Собранный исполняемый файл появится в `src-tauri/target/release/synchro.exe`, а установщик — в `src-tauri/target/release/bundle/nsis/`.

---

## Сообщество и поддержка

- **Официальный Telegram**: [t.me/synchronova](https://t.me/synchronova) — обновления, обсуждения, идеи и помощь.
- **GitHub Issues**: нашли ошибку или хотите предложить улучшение? [Создайте issue](https://github.com/Arean-Max/synchro-nova/issues).

---

## Лицензия

Проект распространяется под свободной лицензией [MIT](LICENSE).
