# Synchro Nova

<div align="center">

[![English](https://img.shields.io/badge/Language-English-blue?style=flat-square)](README.en.md)
[![Telegram](https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white)](https://t.me/synchronova)
[![Releases](https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald)](https://github.com/Arean-Max/synchro-nova/releases)
[![License](https://img.shields.io/badge/License-MIT-white?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20(x64)-informational?style=flat-square)](https://microsoft.com/windows)

**Десктопная утилита для аппаратной цветокоррекции монитора, оптимизации задержки ввода и тонкой настройки Windows.**  
*Написана на Rust и Tauri v2. Потребляет ~2 МБ RAM в трее, использует 0% CPU в простое и на 100% безопасна для античитов.*

<br>

<img src="docs/screenshots/main-frame.png" alt="Synchro Nova Interface" width="880" style="border-radius: 10px; box-shadow: 0 8px 30px rgba(0,0,0,0.5);">

</div>

---

## О проекте

Synchro Nova создавалась как замена громоздким оверлеям и сомнительным «бустерам фпс», которые забивают оперативную память, внедряются в системные библиотеки и вызывают блокировки в соревновательных играх. 

Мы объединили аппаратную регулировку сочности (Digital Vibrance), точную гамма-калибровку, аудит драйверов и прозрачные системные твики в одном лёгком приложении без рекламы, телеметрии и скрытых служб.

---

## Основные возможности

### 1. Аппаратная цветокоррекция (Display Calibration)
- **Прямое управление через Windows GDI & Magnification API**: сочность (vibrance), насыщенность, гамма, контраст и цветовой тон без задержки кадра и без инпут-лага.
- **Режим Black Holo**: интеллектуальное вытягивание контраста и глубоких теней в соревновательных шутерах (CS2, Rust, Apex, Tarkov) без потери деталей в ярких зонах.
- **Автоматические пресеты для игр**: возможность привязать персональную калибровку к конкретной игре.

### 2. Системные твики и оптимизация отклика (System Tweaks)
- **41 проверенная настройка**: отключение телеметрии, настройка системного таймера, приоритеты мультимедийного планировщика (MMCSS), полноэкранная оптимизация (FSO), сетевой стек и TCP autotuning.
- **Гарантированная безопасность (1-Click Rollback)**: перед внесением любых изменений утилита создаёт снимок реестра (`.reg`). В любой момент все параметры можно откатить одной кнопкой.
- **Полная прозрачность**: никаких скрытых скриптов. Каждая команда и путь в реестре описаны в интерфейсе и в [документации по твикам](docs/TWEAKS_REFERENCE.md).

### 3. Анализ драйверов и ПК (Hardware & Drivers)
- Определение видеокарты, текущей версии видеодрайвера и даты его выпуска.
- Проверка актуальности драйверов NVIDIA, AMD, Intel и прямые ссылки на официальные сайты производителей.
- Мониторинг загрузки процессора и оперативной памяти через нативный Windows PDH (Performance Data Helper).

### 4. Полная чистота перед античитами (Anti-Cheat Conformance)
- **Строгий User Mode**: приложение не лезет в чужую память (0 вызовов `PROCESS_VM_READ` или `PROCESS_VM_WRITE`).
- **Без инъекций и перехватов**: 0 вызовов `CreateRemoteThread`, никаких хуков на системный ввод (`SetWindowsHookEx`) и никаких API-detours.
- Безопасно работает параллельно с **Riot Vanguard, Easy Anti-Cheat (EAC), BattlEye, Valve Anti-Cheat (VAC) и Ricochet**.

---

## Производительность и ресурсы

| Показатель | Synchro Nova v2.0 | Типичный софт на Electron |
|---|:---:|:---:|
| **Оперативная память в трее** | **~1.5 – 3 МБ** | 120 – 350 МБ |
| **Нагрузка на CPU в фоне** | **0.0% (0 прерываний таймера)** | 0.5 – 2.5% |
| **Время холодного старта** | **< 0.3 сек** | 2.5 – 6.0 сек |
| **Внешний сетевой трафик** | **0 байт (телеметрия Chromium вырезана)** | Постоянные аналитические запросы |

> [!NOTE]
> В версии 2.0 фоновый цикл проверки калибровки переведён на события Windows `Condvar`. При нейтральном профиле поток полностью засыпает в ядре операционной системы, позволяя процессору переходить в глубокие C-states.

---

## Скачать и установить

Свежие сборки всегда доступны на странице [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Портативная версия (`synchro.exe`)** — запускается сразу из любой папки или с флешки, не требует установки и не оставляет следов в системе.
- **Установщик (`Synchro.Nova_2.0.0_x64-setup.exe`)** — классический установщик с ярлыком на рабочем столе и чистым удалением через «Установка и удаление программ».

### Проверка контрольной суммы (SHA-256)
Для проверки подлинности скачанного файла откройте PowerShell в папке с файлом:
```powershell
Get-FileHash .\synchro.exe -Algorithm SHA256
```
Сверьте полученный хэш со значениями в файле `SHA256SUMS.txt` на странице релиза.

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

# 4. Сборка релизного установщика и бинарника
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
