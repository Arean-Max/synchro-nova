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

### 내가 이것을 만든 이유

CS2 또는 Apex를 시작할 때마다 NVIDIA 제어판을 열고 Digital Vibrance를 최대로 높여 어두운 구석에 있는 적을 찾아낸 다음 게임을 닫은 후 데스크탑과 브라우저가 망막을 태우지 않도록 수동으로 다시 다이얼을 돌려야 했습니다.

싱크로(Synchro)는 이 문제를 완전히 해결합니다.
- **자동 색상 전환**: 게임을 시작하면 색상이 시작됩니다. Alt 키를 누르거나 게임을 닫으면 데스크탑 색상이 즉시 정상으로 돌아갑니다.
- **지연 시간 정리**: Windows 마우스 가속을 종료하고 타이머 해상도를 최적화하며 네트워크 패킷 버퍼링을 조정하여 입력이 즉각적인 느낌을 줍니다.
- **팽만감 제로**: 5MB 미만의 다운로드는 밀리초 안에 시작되며 ~2MB의 RAM을 사용하여 트레이에 조용히 자리잡고 있습니다.

> [!TIP]
> **인기 게임에 대해 테스트된 Vibrance 사전 설정:**
> - **CS2(Dust II, Mirage, Inferno)**: `150~165%`로 설정합니다. 플레이어 모델이 벽과 상자에 섞이는 것을 멈춥니다.
> - **용맹**: '135~140%' 정도가 충분합니다. 게임에는 이미 적 개요 하이라이트가 있습니다. 값이 높을수록 색상이 희미해집니다.
> - **Apex Legends 및 Rust**: `160~170%`는 수풀과 장거리에서 실루엣을 발견하는 데 놀라운 효과를 발휘합니다.
> - **듀얼 모니터**: Vibrance는 게임을 실행하는 기본 모니터에만 적용됩니다. Discord 또는 Chrome의 두 번째 화면은 일반 색상으로 유지됩니다.

---

### 다운로드 및 실행 방법(초보자 가이드)

GitHub를 처음 사용하는 경우 전체 프로세스는 다음과 같습니다.

1. 이 페이지 오른쪽에 있는 [**릴리스**](https://github.com/Arean-Max/synchron-nova/releases)를 클릭하세요.
2. **`synchro.exe`**를 다운로드하세요 — 휴대용 버전입니다. 어디에나 저장하고 두 번 클릭하여 실행하면 설치 프로그램이 필요하지 않습니다. (또는 시작 메뉴 바로가기를 원하면 'Synchro.Nova_*_x64-setup.exe'를 선택하세요).
3. **Windows에서 파란색 SmartScreen 팝업("Windows가 PC를 보호했습니다")이 표시되는 경우**:
   - 괜찮아요. Windows는 Microsoft로부터 연간 500달러의 기업 서명을 구매하지 않는 무료 인디 프로그램에 대해 이를 보여줍니다.
   - **"추가 정보"** ➔ **"어쨌든 실행"**을 클릭하세요.
   - 이 프로젝트는 바로 여기서 100% 오픈 소스입니다. 원격 측정 없음, 채굴자 없음, 광고 없음.

---

### 빠른 시작: 간단한 3단계

1. `synchro.exe`를 실행하세요.
2. **"색상"** 탭에서 활기 수준(예: `150%`)을 선택합니다.
3. 목록에서 게임을 선택합니다(또는 `.exe` 파일을 가리킵니다).
트레이로 최소화하여 플레이하세요. 다시는 수동으로 조정할 필요가 없습니다.

---

### FAQ

#### CS2, Valorant, Rust 또는 Faceit에서 금지되나요?
**100% 아니요. 금지 위험은 0입니다.**  
Synchro Nova **게임 RAM을 건드리지 않고 프로세스에 DLL을 삽입하지 않으며 오버레이를 그리지 않습니다.**. NVIDIA 또는 AMD 제어판에서 슬라이더를 드래그하는 것과 마찬가지로 표준 Windows 디스플레이 API를 통해서만 감마를 조정합니다. 치트 방지 시스템(VAC, Faceit, Vanguard, EAC, BattlEye)은 이를 게임 상호 작용으로 간주하지도 않습니다.

#### NVIDIA/AMD 제어판에서 이것을 사용하는 이유는 무엇입니까?
GPU 제어판은 게임별 설정을 자동으로 전환할 수 없습니다. 여기에서 활력을 높이면 비디오를 검색하거나 시청하는 동안 전체 데스크톱이 과포화 상태로 유지됩니다. 싱크로는 이를 즉시 자동으로 처리합니다.

#### Windows 조정은 안전합니까?
예. 우리는 핵심 OS 무결성을 건드리지 않고 Windows Defender를 손상시키지 않으며 페이지 파일을 비활성화하지 않습니다. 설정을 터치하기 전에 Syncro는 Windows 복원 지점과 레지스트리 백업을 생성합니다. 조정이 마음에 들지 않으면 **"변경 사항 롤백"**을 클릭하면 모든 내용이 즉시 되돌려집니다.

---

### 문서화 및 심층 분석

심층적인 기술 세부정보를 보려면 당사의 전용 리소스를 확인하세요.
- 📖 [DeepWiki의 전체 대화형 문서](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [GitDiagram의 아키텍처 다이어그램](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [완전한 조정 및 레지스트리 참조 (TWEAKS_REFERENCE.ko.md)](docs/TWEAKS_REFERENCE.ko.md)

---

### 소스에서 빌드(개발자용)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # 개발 모드
npm run build     # 릴리스 빌드
```

---

### 기여자

- [sotoykor-source](https://github.com/sotoykor-source) — 원본 작성자 및 업스트림 작성자
- [Arean-Max](https://github.com/Arean-Max) — 개발 및 유지보수

---

### 특허

[MIT](LICENSE) — 자유롭게 사용, 수정, 공유할 수 있습니다.
