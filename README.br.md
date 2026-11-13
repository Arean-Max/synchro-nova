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

### Por que eu construí isso

Cada vez que eu lançava o CS2 ou o Apex, eu tinha que abrir o Painel de Controle da NVIDIA, aumentar o Digital Vibrance para detectar inimigos em cantos escuros e, em seguida, reduzi-lo manualmente após fechar o jogo para que minha área de trabalho e navegador não queimassem minhas retinas.

Synchro resolve isso de uma vez por todas:
- **Troca automática de cores**: Inicie o jogo, as cores entram em ação. Alt-Tab ou feche o jogo, as cores da área de trabalho voltam imediatamente ao normal.
- **Limpeza de latência**: Elimina a aceleração do mouse do Windows, otimiza a resolução do temporizador e reduz o buffer de pacotes de rede para que a entrada pareça instantânea.
- **Inchaço zero**: O download com menos de 5 MB começa em milissegundos e fica silenciosamente na bandeja usando aproximadamente 2 MB de RAM.

> [!TIP]
> **Predefinições de vibração testadas para jogos populares:**
> - **CS2 (Dust II, Mirage, Inferno)**: Defina como `150–165%`. Os modelos dos jogadores param de se misturar com paredes e caixotes.
> - **Valorant**: Cerca de `135–140%` é suficiente. O jogo já possui destaques do contorno do inimigo; valores mais altos apenas desbotam as cores.
> - **Apex Legends & Rust**: `160–170%` faz maravilhas para detectar silhuetas em arbustos e a longas distâncias.
> - **Monitores duplos**: a vibração se aplica apenas ao monitor principal que executa o jogo. Sua segunda tela com Discord ou Chrome permanece com as cores normais.

---

### Como baixar e executar (guia para iniciantes)

Se você é novo no GitHub, aqui está todo o processo:

1. Clique em [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) no lado direito desta página.
2. Baixe **`synchro.exe`** — é a versão portátil. Salve-o em qualquer lugar e clique duas vezes para iniciar, sem necessidade de instalador. (Ou pegue `Synchro.Nova_*_x64-setup.exe` se desejar atalhos do menu Iniciar).
3. **Se o Windows mostrar um pop-up azul do SmartScreen ("O Windows protegeu seu PC")**:
   - Não se preocupe. O Windows mostra isso para qualquer programa independente gratuito que não compre uma assinatura corporativa de US$ 500/ano da Microsoft.
   - Clique em **"Mais informações"** ➔ **"Executar mesmo assim"**.
   - O projeto é 100% open source aqui mesmo. Zero telemetria, zero mineradores, zero anúncios.

---

### Início rápido: 3 etapas simples

1. Inicie `synchro.exe`.
2. Na guia **"Cor"**, escolha seu nível de vibração (por exemplo, `150%`).
3. Selecione seu jogo na lista (ou aponte para o arquivo `.exe`).
Minimize para a bandeja e jogue. Você nunca mais precisará ajustá-lo manualmente.

---

### Perguntas frequentes

#### Serei banido no CS2, Valorant, Rust ou Faceit?
**100% Não. O risco de banimento é zero.**  
Synchro Nova **não toca na RAM do jogo, não injeta DLLs em processos e não desenha sobreposições**. Ele ajusta a gama puramente por meio de APIs de exibição padrão do Windows – exatamente como arrastar o controle deslizante no painel de controle NVIDIA ou AMD. Os sistemas anti-cheat (VAC, Faceit, Vanguard, EAC, BattlEye) nem sequer veem isso como uma interação de jogo.

#### Por que usar isso no painel de controle NVIDIA/AMD?
Os painéis de controle da GPU não podem alternar automaticamente as configurações por jogo. Se você aumentar a vibração, toda a sua área de trabalho ficará saturada enquanto você navega ou assiste a vídeos. Synchro lida com isso automaticamente em tempo real.

#### Os ajustes do Windows são seguros?
Sim. Não alteramos a integridade central do sistema operacional, não quebramos o Windows Defender e não desabilitamos seu arquivo de paginação. Antes de alterar qualquer configuração, o Synchro cria um ponto de restauração do Windows e um backup do registro. Se você não gostar de ajustes, clicar em **"Reverter alterações"** reverte tudo imediatamente.

---

### Documentação e aprofundamento

Para detalhes técnicos detalhados, verifique nossos recursos dedicados:
- 📖 [Documentação interativa completa no DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Diagrama de arquitetura no GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Ajustes completos e referência de registro (TWEAKS_REFERENCE.br.md)](docs/TWEAKS_REFERENCE.br.md)

---

### Construir a partir do código-fonte (para desenvolvedores)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       #modo de desenvolvimento
npm run build     # versão de lançamento
```

---

### Colaboradores

- [sotoykor-source](https://github.com/sotoykor-source) — Criador original e autor upstream
- [Arean-Max](https://github.com/Arean-Max) — Desenvolvimento e manutenção

---

### Licença

[MIT](LICENSE) — livre para usar, modificar e compartilhar.
