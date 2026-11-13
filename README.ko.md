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

### 싱크로노바는 무슨 일을 하나요?

간단히 말해서, 게임의 색상 선명도와 대비를 높여 어두운 환경에서 적들이 눈에 띄게 만드는 경량의 무료 유틸리티입니다.

- **자동 게임별 프로필**: 게임에 대해 가장 좋아하는 활기 수준을 한 번 설정하세요. CS2, Valorant 또는 Apex를 시작하면 색상이 자동으로 시작됩니다. Alt 키를 누르거나 종료하면 디스플레이 색상이 즉시 정상으로 돌아가므로 데스크탑이나 브라우저에서 눈이 아프지 않습니다.
- **지연 시간 및 응답성**: 테스트를 거쳐 안전한 Windows 응답성 조정(타이머 해상도, 원시 마우스 입력, 네트워크 패킷 처리)이 포함되어 있어 입력이 빠르게 느껴집니다.
- **제로 리소스 드래그**: 전체 앱은 5MB 미만이고, 밀리초 안에 열리며, 단지 ~2MB의 RAM을 사용하여 트레이에 위치합니다.

---

### 다운로드 및 실행 방법(초보자 가이드)

GitHub를 처음 방문하는 경우 정확히 수행할 작업은 다음과 같습니다.

1. 다음으로 향하세요. [**Releases**](https://github.com/Arean-Max/synchro-nova/releases) 페이지를 방문하세요(또는 오른쪽 사이드바에서 "릴리스"를 클릭하세요).
2. 최신 릴리스에서 다음을 다운로드하십시오.
   - **`synchro.exe`** — **휴대용 버전(권장)**. 아무 폴더에나 다운로드하고 두 번 클릭하여 실행하면 됩니다. 설정이 필요하지 않습니다.
   - 또는 **`Synchro.Nova_*_x64-setup.exe`** — 바탕 화면 및 시작 메뉴 바로 가기를 선호하는 경우 표준 Windows 설치 프로그램.
3. **Windows에서 파란색 팝업이 표시되는 경우("Windows가 PC를 보호했습니다"/SmartScreen)**:
   - 이는 Microsoft에서 연간 500달러의 값비싼 기업 인증서를 구입하지 않는 모든 무료 오픈 소스 도구에서 발생합니다.
   - 딸깍 하는 소리 **"추가 정보"**, 그런 다음 클릭 **"어쨌든 실행"**.
   - 전체 소스 코드는 바로 여기 GitHub에 100% 공개되어 있습니다. 애드웨어도, 원격 측정도, 넌센스도 없습니다.

---

### 빠른 시작(1분 설정)

1. 시작하다 **Synchro Nova**.
2. 로 이동 **"색상"** 탭을 클릭하고 활기 수준을 선택하세요(예: `150%` 또는 `200%`).
3. 게임에 연결하세요(앱이 실행 중인 게임을 자동으로 감지하거나 `.exe` 수동으로).
4. 그게 다야! 트레이로 최소화해서 놀아보세요.

---

### FAQ

#### CS2, Valorant, Rust 또는 Faceit에서 금지되나요?
**100% 아니요. 금지 위험은 0입니다.**  
Synchro Nova **게임 메모리를 건드리지 않고, 프로세스에 DLL을 삽입하지 않으며, 게임 오버레이를 그리지 않습니다.**. 표준 Windows API(DWM 및 GPU 드라이버)를 통해서만 데스크탑 감마를 조정합니다. 이는 NVIDIA 또는 AMD 제어판에서 Digital Vibrance 슬라이더를 드래그하는 것과 똑같습니다. 치트 방지 엔진(VAC, Faceit, Vanguard, Easy Anti-Cheat, BattlEye)은 데스크탑 색상 변경에 플래그를 지정하지 않습니다.

#### NVIDIA 또는 AMD 제어판을 사용하지 않는 이유는 무엇입니까?
GPU 제어판은 게임을 시작하거나 닫을 때 색상을 자동으로 전환하지 않습니다. 여기서 활력을 높이면 전체 Windows 데스크톱과 브라우저가 과포화 상태를 유지하게 됩니다. 싱크로는 이를 즉시 자동으로 처리합니다. 게임 실행 = 풍부한 색상; 게임 종료 = 깨끗한 바탕 화면 색상.

#### 시스템을 조정하면 Windows가 엉망이 될까요?
아니요. 모든 조정은 잘 문서화된 표준 Windows 매개변수를 사용합니다. 무엇이든 수정하기 전에 Synro Nova는 Windows 복원 지점과 수정된 레지스트리 키의 백업을 생성합니다. 클릭하면 언제든지 모든 것을 Windows 기본값으로 되돌릴 수 있습니다. **"롤백 변경 사항"**.

---

### 문서화 및 심층 분석

이 페이지를 깔끔하고 읽기 쉽게 유지하기 위해 광범위한 기술 문서가 전용 위키에 있습니다.
- 📖 [DeepWiki의 전체 대화형 문서](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram의 아키텍처 다이어그램](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [완전한 조정 및 레지스트리 참조 (TWEAKS_REFERENCE.md)](docs/TWEAKS_REFERENCE.md)

---

### 소스에서 빌드(개발자용)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # run in development mode
npm run build     # build production release exe
```

---

### 기여자

- [sotoykor-source](https://github.com/sotoykor-source) — 원본 작성자 및 업스트림 작성자
- [Arean-Max](https://github.com/Arean-Max) — 개발 및 유지보수

---

### 특허

아래에 배포됨 [MIT License](LICENSE).
