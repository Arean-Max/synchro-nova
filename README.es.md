<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Herramienta ligera para saturación de color del monitor y optimización de latencia en Windows.</p>

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

### ¿Por qué construí esto?

Cada vez que iniciaba CS2 o Apex, tenía que abrir el Panel de control de NVIDIA, aumentar Digital Vibrance para detectar enemigos en rincones oscuros y luego volver a bajarlo manualmente después de cerrar el juego para que mi escritorio y mi navegador no quemaran mis retinas.

Synchro soluciona esto de una vez por todas:
- **Cambio de color automático**: Inicie su juego, los colores aparecen. Presione Alt-Tab o cierre el juego, los colores de su escritorio vuelven inmediatamente a la normalidad.
- **Limpieza de latencia**: Elimina la aceleración del mouse de Windows, optimiza la resolución del temporizador y recorta el almacenamiento en búfer de los paquetes de red para que la entrada se sienta instantánea.
- **Cero hinchazón**: Descarga de menos de 5 MB, comienza en milisegundos y permanece silenciosamente en la bandeja usando ~2 MB de RAM.

> [!TIP]
> **Ajustes preestablecidos de intensidad probados para juegos populares:**
> - **CS2 (Dust II, Mirage, Inferno)**: Establecido en `150–165%`. Los modelos de jugadores dejan de mezclarse con paredes y cajas.
> - **Valorant**: Alrededor de "135-140%" es suficiente. El juego ya tiene aspectos destacados del contorno del enemigo; los valores más altos simplemente eliminan los colores.
> - **Apex Legends & Rust**: `160–170%` funciona de maravilla para detectar siluetas en los arbustos y a largas distancias.
> - **Monitores duales**: La intensidad solo se aplica al monitor principal que ejecuta tu juego. Tu segunda pantalla con Discord o Chrome permanece en colores normales.

---

### Cómo descargar y ejecutar (Guía para principiantes)

Si eres nuevo en GitHub, aquí tienes el proceso completo:

1. Haga clic en [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) en el lado derecho de esta página.
2. Descargue **`synchro.exe`**: es la versión portátil. Guárdelo en cualquier lugar y haga doble clic para iniciarlo, no se requiere instalador. (O toma `Synchro.Nova_*_x64-setup.exe` si quieres accesos directos al menú Inicio).
3. **Si Windows muestra una ventana emergente azul de SmartScreen ("Windows protegió su PC")**:
   - No te preocupes. Windows muestra esto para cualquier programa independiente gratuito que no compre una firma corporativa de Microsoft por valor de 500 dólares al año.
   - Haga clic en **"Más información"** ➔ **"Ejecutar de todos modos"**.
   - El proyecto es 100% de código abierto aquí mismo. Cero telemetría, cero mineros, cero anuncios.

---

### Inicio rápido: 3 sencillos pasos

1. Inicie `synchro.exe`.
2. En la pestaña **"Color"**, elige tu nivel de intensidad (por ejemplo, `150%`).
3. Selecciona tu juego de la lista (o señala su archivo `.exe`).
Minimizar a la bandeja y jugar. Nunca más tendrás que ajustarlo manualmente.

---

### Preguntas frecuentes

#### ¿Me banearán en CS2, Valorant, Rust o Faceit?
**100% No. El riesgo de prohibición es cero.**  
Synchro Nova **no toca la RAM del juego, no inyecta archivos DLL en los procesos y no dibuja superposiciones**. Ajusta la gamma únicamente a través de las API de visualización estándar de Windows, exactamente como arrastrar el control deslizante en el panel de control de NVIDIA o AMD. Los sistemas antitrampas (VAC, Faceit, Vanguard, EAC, BattlEye) ni siquiera lo ven como una interacción de juego.

#### ¿Por qué utilizar esto en el Panel de control de NVIDIA/AMD?
Los paneles de control de la GPU no pueden alternar automáticamente la configuración por juego. Si aumenta la intensidad allí, todo su escritorio permanece sobresaturado mientras navega o mira videos. Synchro maneja esto automáticamente sobre la marcha.

#### ¿Son seguros los ajustes de Windows?
Sí. No tocamos la integridad central del sistema operativo, no dañamos Windows Defender y no deshabilitamos su archivo de paginación. Antes de tocar cualquier configuración, Synchro crea un punto de restauración de Windows y una copia de seguridad del registro. Si no le gusta un ajuste, al hacer clic en **"Revertir cambios"** se revierte todo inmediatamente.

---

### Documentación y análisis profundo

Para obtener detalles técnicos detallados, consulte nuestros recursos dedicados:
- 📖 [Documentación interactiva completa en DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagrama de arquitectura en GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Ajustes completos y referencia de registro (TWEAKS_REFERENCE.es.md)](docs/TWEAKS_REFERENCE.es.md)

---

### Construir desde fuente (para desarrolladores)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # modo de desarrollo
npm run build     # versión de lanzamiento
```

---

### Colaboradores

- [sotoykor-source](https://github.com/sotoykor-source) — Creador original y autor ascendente
- [Arean-Max](https://github.com/Arean-Max) — Desarrollo y mantenimiento

---

### Licencia

[MIT](LICENSE) — Gratis para usar, modificar y compartir.
