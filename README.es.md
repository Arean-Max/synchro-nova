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

### ¿Qué hace Synchro Nova?

En resumen: es una utilidad ligera y gratuita que aumenta la intensidad y el contraste del color en los juegos, ayudando a que los enemigos se destaquen en entornos oscuros.

- **Perfiles automáticos por juego**: Establece tu nivel de intensidad favorito para tu juego una vez. Cuando inicias CS2, Valorant o Apex, los colores aparecen automáticamente. Cuando presionas la tecla Alt o sales, los colores de la pantalla vuelven inmediatamente a la normalidad para que no te duelan los ojos en el escritorio o en el navegador.
- **Latencia y capacidad de respuesta**: Incluye ajustes de capacidad de respuesta de Windows seguros y probados (resolución del temporizador, entrada sin formato del mouse, manejo de paquetes de red) para que la entrada se sienta ágil.
- **Arrastre de recursos cero**: La aplicación completa tiene menos de 5 MB, se abre en milisegundos y se ubica en la bandeja con solo ~2 MB de RAM.

---

### Cómo descargar y ejecutar (Guía para principiantes)

Si visita GitHub por primera vez, esto es exactamente lo que debe hacer:

1. Dirígete al [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) página (o haga clic en "Versiones" en la barra lateral derecha).
2. En la última versión, descargue:
   - **`synchro.exe`** — **versión portátil (recomendada)**. Simplemente descárguelo a cualquier carpeta y haga doble clic para ejecutarlo. No se necesita configuración.
   - O **`Synchro.Nova_*_x64-setup.exe`** — instalador estándar de Windows si prefiere los accesos directos al escritorio y al menú de inicio.
3. **Si Windows muestra una ventana emergente azul ("Windows protegió su PC"/SmartScreen)**:
   - Esto sucede con todas las herramientas gratuitas de código abierto que no compran un costoso certificado empresarial de Microsoft de 500 dólares al año.
   - Hacer clic **"Más información"**, luego haga clic **"Corre de todos modos"**.
   - Todo el código fuente está 100% abierto aquí en GitHub: sin publicidad, sin telemetría, sin tonterías.

---

### Inicio rápido (configuración de 1 minuto)

1. Lanzamiento **Synchro Nova**.
2. Ir al **"Color"** pestaña y elija su nivel de intensidad (p. ej. `150%` o `200%`).
3. Vincúlalo a tu juego (la aplicación detecta automáticamente los juegos en ejecución, o puedes elegir el `.exe` a mano).
4. ¡Eso es todo! Minimizar a la bandeja y jugar.

---

### Preguntas frecuentes

#### ¿Me banearán en CS2, Valorant, Rust o Faceit?
**100% No. El riesgo de prohibición es cero.**  
Synchro Nova **no toca la memoria del juego, no inyecta archivos DLL en los procesos y no dibuja superposiciones del juego**. Solo ajusta la gamma del escritorio a través de las API estándar de Windows (controladores DWM y GPU), exactamente como arrastrar el control deslizante Digital Vibrance en el panel de control de NVIDIA o AMD. Los motores antitrampas (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) no marcan los cambios de color del escritorio.

#### ¿Por qué no utilizar simplemente el Panel de control de NVIDIA o AMD?
Los paneles de control de la GPU no alternan colores automáticamente cuando inicias o cierras un juego. Si aumenta la vitalidad allí, todo su escritorio y navegador de Windows permanecerán sobresaturados. Synchro maneja esto automáticamente sobre la marcha: juego en ejecución = colores intensos; juego cerrado = colores de escritorio limpios.

#### ¿Los ajustes del sistema arruinarán mi Windows?
No. Todos los ajustes utilizan parámetros estándar de Windows bien documentados. Antes de modificar cualquier cosa, Synchro Nova crea un punto de restauración de Windows y una copia de seguridad de las claves de registro modificadas. Puede revertir todo a los valores predeterminados de Windows en cualquier momento haciendo clic **"Revertir cambios"**.

---

### Documentación y análisis profundo

Para mantener esta página limpia y fácil de leer, nuestra extensa documentación técnica se encuentra en nuestra wiki dedicada:
- 📖 [Documentación interactiva completa en DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagrama de arquitectura en GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Ajustes completos y referencia de registro (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Construir desde fuente (para desarrolladores)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Colaboradores

- [sotoykor-source](https://github.com/sotoykor-source) — Creador original y autor ascendente
- [Arean-Max](https://github.com/Arean-Max) — Desarrollo y mantenimiento

---

### Licencia

Distribuido bajo el [MIT License](LICENSE).
