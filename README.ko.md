<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">모니터 채도 조절 및 Windows 지연 시간 최적화를 위한 경량 유틸리티.</p>

<p align="center">
  <a href="https://t.me/synchronova"><img alt="Telegram" src="https://img.shields.io/badge/Telegram-Channel-229ED9?style=flat-square&logo=telegram&logoColor=white" /></a>
  <a href="https://deepwiki.com/Arean-Max/synchro-nova"><img alt="Docs" src="https://img.shields.io/badge/Docs-DeepWiki-blueviolet?style=flat-square" /></a>
  <a href="https://gitdiagram.com/arean-max/synchro-nova"><img alt="Architecture" src="https://img.shields.io/badge/Diagram-GitDiagram-orange?style=flat-square" /></a>
  <a href="https://github.com/Arean-Max/synchro-nova/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/synchro-nova?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-AGPL--3.0-blue?style=flat-square" /></a>
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

### 설치

[**Releases**](https://github.com/Arean-Max/synchro-nova/releases)에서 최신 빌드를 다운로드하세요:

- **포터블 버전 (`synchro.exe`)** — 압축을 풀고 `synchro.exe`를 실행하세요. 설치가 필요 없습니다.
- **설치 프로그램 (`Synchro.Nova_*_x64-setup.exe`)** — 바로가기 및 제거 프로그램을 포함한 기본 설치 파일.

---

### 자주 묻는 질문 (FAQ)

#### 어떤 프로그램인가요?
경쟁 게임을 위한 가벼운 모니터 튜닝 도구입니다. 게임 속 색감을 생생하게 만들고 적 식별을 쉽게 돕고 불필요한 Windows 인풋렉을 줄여줍니다. 게임 실행 시 자동으로 프로필이 적용됩니다.

#### 안티치트에 걸리나요?
아닙니다. Synchro Nova는 게임 메모리에 접근하지 않으며 DLL을 인젝션하거나 오버레이를 띄우지 않습니다. NVIDIA나 AMD 패널과 동일하게 표준 Windows WinAPI로 디스플레이를 제어하므로 VAC, Faceit, Vanguard, EAC, BattlEye에서 100% 안전합니다.

#### Synchro Nova는 일반 그래픽카드 제어판보다 무엇이 좋나요?
Rust + Tauri 기반으로 제작되어 즉시 실행되며, 트레이에서 0% CPU와 약 4MB 메모리만 사용합니다. 게임별 자동 프로필 전환과 안전한 시스템 지연 최적화를 지원합니다.

#### 윈도우 트윅으로 시스템이 고장 나지 않나요?
안전합니다. 철저히 검증된 설정만 적용되며, 변경 전 시스템 복원 지점과 레지스트리 백업을 자동으로 생성하여 언제든 원클릭으로 되돌릴 수 있습니다.

---

### 소스코드 빌드

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 개발 모드로 실행
npm run build     # 설치 파일 빌드
```

---

### 라이선스

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
