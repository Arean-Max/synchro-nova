<p align="center">
  <a href="https://github.com/Arean-Max/synchro-nova">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Synchro Nova logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Công cụ màn hình siêu nhẹ để tăng độ rực rỡ màu sắc và tối ưu hóa độ trễ trên Windows.</p>

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

### Cài đặt

Tải bản dựng mới nhất tại trang [**Releases**](https://github.com/Arean-Max/synchro-nova/releases):

- **Bản Portable (`synchro.exe`)** — Giải nén và chạy `synchro.exe`. Không cần cài đặt.
- **Bộ cài đặt (`Synchro.Nova_*_x64-setup.exe`)** — Bộ cài Windows tiêu chuẩn có phím tắt và trình gỡ cài đặt.

---

### Câu hỏi thường gặp (FAQ)

#### Công cụ này là gì?
Một tiện ích nhẹ dành cho game thủ. Giúp màu sắc trong game trở nên sống động, giúp phát hiện kẻ địch dễ dàng hơn và giảm độ trễ dư thừa của Windows. Tự động chuyển đổi cấu hình khi mở và đóng game.

#### Có bị cấm (ban) trong game không?
Không hề. Synchro Nova không can thiệp vào bộ nhớ game, không chèn DLL và không vẽ overlay. Màu sắc được điều chỉnh thông qua WinAPI chính thức của Windows, tương tự bảng điều khiển NVIDIA/AMD. Hoàn toàn an toàn với VAC, Faceit, Vanguard, EAC và BattlEye.

#### Có gì vượt trội hơn VibranceGUI?
Được viết bằng Rust + Tauri: khởi động tức thì, tiêu tốn 0% CPU và chỉ khoảng 4 MB RAM ở khay hệ thống. Hỗ trợ tự động chuyển đổi cấu hình theo game cùng các tinh chỉnh độ trễ an toàn.

#### Tinh chỉnh có làm lỗi Windows không?
Không. Trước mỗi lần thay đổi, ứng dụng tự động tạo Điểm khôi phục hệ thống (Restore Point) và sao lưu registry để bạn có thể hoàn tác bất cứ lúc nào.

---

### Build từ mã nguồn

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       # chạy ở chế độ phát triển
npm run build     # build bộ cài đặt phát hành
```

---

### Giấy phép

[AGPL-3.0](LICENSE)

---

### Documentation & Architecture

- [Documentation on DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- [Architecture Diagram on GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
