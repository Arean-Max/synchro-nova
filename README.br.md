<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Utilitário leve para saturação de cores do monitor e otimização de latência no Windows.</p>

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

### O que Synchro Nova faz?

Resumindo: é um utilitário leve e gratuito que aumenta a vibração e o contraste das cores nos jogos, ajudando os inimigos a se destacarem em ambientes escuros.

- **Perfis automáticos por jogo**: Defina seu nível de vibração favorito para o seu jogo uma vez. Quando você inicia CS2, Valorant ou Apex, as cores aparecem automaticamente. Quando você pressiona Alt-Tab ou sai, as cores da tela retornam imediatamente ao normal para que seus olhos não doam na área de trabalho ou no navegador.
- **Latência e capacidade de resposta**: Inclui ajustes testados e seguros de capacidade de resposta do Windows (resolução do temporizador, entrada bruta do mouse, manipulação de pacotes de rede) para tornar a entrada mais rápida.
- **Arraste zero de recursos**: O aplicativo inteiro tem menos de 5 MB, abre em milissegundos e fica na bandeja usando apenas cerca de 2 MB de RAM.

---

### Como baixar e executar (guia para iniciantes)

Se você estiver visitando o GitHub pela primeira vez, aqui está exatamente o que fazer:

1. Vá para o [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) página (ou clique em "Lançamentos" na barra lateral direita).
2. Na versão mais recente, baixe:
   - **`synchro.exe`** — **versão portátil (recomendado)**. Basta baixá-lo para qualquer pasta e clicar duas vezes para executá-lo. Nenhuma configuração necessária.
   - Ou **`Synchro.Nova_*_x64-setup.exe`** — instalador padrão do Windows se você preferir atalhos na área de trabalho e no menu Iniciar.
3. **Se o Windows mostrar um pop-up azul ("O Windows protegeu seu PC" / SmartScreen)**:
   - Isso acontece com todas as ferramentas gratuitas de código aberto que não compram um caro certificado empresarial de US$ 500/ano da Microsoft.
   - Clique **"Mais informações"**, então clique **"Corra de qualquer maneira"**.
   - Todo o código-fonte é 100% aberto aqui mesmo no GitHub — sem adware, sem telemetria, sem bobagens.

---

### Início rápido (configuração de 1 minuto)

1. Lançar **Synchro Nova**.
2. Vá para o **"Cor"** guia e escolha seu nível de vibração (por exemplo, `150%` ou `200%`).
3. Vincule-o ao seu jogo (o aplicativo detecta automaticamente os jogos em execução ou você pode escolher o `.exe` manualmente).
4. É isso! Minimize na bandeja e jogue.

---

### Perguntas frequentes

#### Serei banido no CS2, Valorant, Rust ou Faceit?
**100% Não. O risco de banimento é zero.**  
Synchro Nova **não toca na memória do jogo, não injeta DLLs em processos e não desenha sobreposições de jogos**. Ele ajusta apenas a gama do desktop por meio de APIs padrão do Windows (drivers DWM e GPU) – exatamente como arrastar o controle deslizante Digital Vibrance no painel de controle NVIDIA ou AMD. Mecanismos anti-cheat (VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye) não sinalizam alterações nas cores da área de trabalho.

#### Por que não usar apenas o painel de controle NVIDIA ou AMD?
Os painéis de controle da GPU não alternam as cores automaticamente quando você inicia ou fecha um jogo. Se você aumentar a vibração, toda a área de trabalho e o navegador do Windows ficarão saturados. O Synchro lida com isso automaticamente em tempo real: jogo em execução = cores ricas; jogo fechado = cores limpas da área de trabalho.

#### Os ajustes do sistema atrapalharão meu Windows?
Não. Todos os ajustes usam parâmetros padrão e bem documentados do Windows. Antes de modificar qualquer coisa, o Synchro Nova cria um ponto de restauração do Windows e um backup das chaves de registro modificadas. Você pode reverter tudo para os padrões do Windows a qualquer momento clicando em **"Alterações de reversão"**.

---

### Documentação e aprofundamento

Para manter esta página limpa e fácil de ler, uma extensa documentação técnica está disponível em nosso wiki dedicado:
- 📖 [Documentação interativa completa no DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagrama de arquitetura no GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Ajustes completos e referência de registro (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### Construir a partir do código-fonte (para desenvolvedores)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### Colaboradores

- [sotoykor-source](https://github.com/sotoykor-source) — Criador original e autor upstream
- [Arean-Max](https://github.com/Arean-Max) — Desenvolvimento e manutenção

---

### Licença

Distribuído sob o [MIT License](LICENSE).
