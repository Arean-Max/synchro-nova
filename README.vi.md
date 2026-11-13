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

### Tại sao tôi xây dựng cái này

Mỗi lần tôi khởi chạy CS2 hoặc Apex, tôi phải mở Bảng điều khiển NVIDIA, bật Digital Vibrance để phát hiện kẻ thù trong góc tối, sau đó quay số lại theo cách thủ công sau khi đóng trò chơi để máy tính để bàn và trình duyệt không làm cháy võng mạc của tôi.

Synchro giải quyết vấn đề này một lần và mãi mãi:
- **Tự động chuyển màu**: Khởi chạy trò chơi của bạn, màu sắc bắt đầu xuất hiện. Alt-Tab hoặc đóng trò chơi, màu sắc trên màn hình của bạn ngay lập tức trở lại bình thường.
- **Dọn dẹp độ trễ**: Loại bỏ khả năng tăng tốc chuột của Windows, tối ưu hóa độ phân giải của bộ hẹn giờ và cắt bớt bộ đệm gói mạng để đầu vào có cảm giác tức thì.
- **Không đầy hơi**: Tải xuống dưới 5 MB, bắt đầu sau một phần nghìn giây và nằm yên lặng trong khay sử dụng ~ 2 MB RAM.

> [!TIP]
> **Đã thử nghiệm cài đặt trước độ rung cho các trò chơi phổ biến:**
> - **CS2 (Dust II, Mirage, Inferno)**: Đặt thành `150–165%`. Mô hình người chơi ngừng hòa vào tường và thùng.
> - **Có giá trị**: Khoảng `135–140%` là nhiều. Trò chơi đã có những điểm nổi bật về phác thảo kẻ thù; giá trị cao hơn chỉ làm mất đi màu sắc.
> - **Apex Legends & Rust**: `160–170%` có tác dụng kỳ diệu trong việc phát hiện bóng trong bụi rậm và ở khoảng cách xa.
> - **Màn hình kép**: Độ rung chỉ áp dụng cho màn hình chính đang chạy trò chơi của bạn. Màn hình thứ hai của bạn với Discord hoặc Chrome vẫn giữ nguyên màu sắc bình thường.

---

### Cách tải xuống và chạy (Hướng dẫn cho người mới bắt đầu)

Nếu bạn chưa quen với GitHub, đây là toàn bộ quá trình:

1. Nhấp vào [**Release**](https://github.com/Arean-Max/synchro-nova/releases) ở bên phải trang này.
2. Tải xuống **`synchro.exe`** — đây là phiên bản di động. Lưu nó ở bất cứ đâu và nhấp đúp để khởi chạy, không cần cài đặt. (Hoặc lấy `Synchro.Nova_*_x64-setup.exe` nếu bạn muốn có các phím tắt menu Bắt đầu).
3. **Nếu Windows hiển thị cửa sổ bật lên SmartScreen màu xanh ("Windows bảo vệ PC của bạn")**:
   - Đừng lo lắng. Windows hiển thị điều này cho bất kỳ chương trình độc lập miễn phí nào không mua chữ ký công ty trị giá 500 USD/năm từ Microsoft.
   - Nhấp vào **"Thông tin thêm"** ➔ **"Vẫn chạy"**.
   - Dự án là nguồn mở 100% ngay tại đây. Không đo từ xa, không khai thác, không quảng cáo.

---

### Bắt đầu nhanh: 3 bước đơn giản

1. Khởi chạy `synchro.exe`.
2. Trong tab **"Màu"**, chọn mức độ rung của bạn (ví dụ: `150%`).
3. Chọn trò chơi của bạn từ danh sách (hoặc trỏ tới tệp `.exe` của nó).
Thu nhỏ vào khay và chơi. Bạn không bao giờ phải điều chỉnh nó bằng tay nữa.

---

### Câu hỏi thường gặp

#### Tôi có bị cấm chơi CS2, Valorant, Rust hay Faceit không?
**100% Không. Rủi ro cấm là bằng không.**  
Synchro Nova **không chạm vào RAM trò chơi, không đưa DLL vào các tiến trình và không vẽ lớp phủ**. Nó điều chỉnh gamma hoàn toàn thông qua các API hiển thị tiêu chuẩn của Windows — giống hệt như việc kéo thanh trượt trong bảng điều khiển NVIDIA hoặc AMD. Các hệ thống chống gian lận (VAC, Faceit, Vanguard, EAC, BattlEye) thậm chí không coi đó là sự tương tác trong trò chơi.

#### Tại sao nên sử dụng tính năng này trên Bảng điều khiển NVIDIA / AMD?
Bảng điều khiển GPU không thể tự động chuyển đổi cài đặt cho mỗi trò chơi. Nếu bạn chỉnh độ rung ở đó, toàn bộ màn hình của bạn sẽ bị bão hòa trong khi duyệt hoặc xem video. Synchro tự động xử lý việc này một cách nhanh chóng.

#### Các tinh chỉnh Windows có an toàn không?
Đúng. Chúng tôi không chạm đến tính toàn vẹn cốt lõi của hệ điều hành, chúng tôi không phá vỡ Windows Defender và chúng tôi không vô hiệu hóa tệp trang của bạn. Trước khi chạm vào bất kỳ cài đặt nào, Synchro sẽ tạo Điểm khôi phục Windows và bản sao lưu sổ đăng ký. Nếu bạn không thích một chỉnh sửa nào, hãy nhấp vào **"Phục hồi các thay đổi"** để hoàn nguyên mọi thứ ngay lập tức.

---

### Tài liệu & Tìm hiểu sâu

Để biết chi tiết kỹ thuật chuyên sâu, hãy kiểm tra các tài nguyên chuyên dụng của chúng tôi:
- 📖 [Tài liệu tương tác đầy đủ trên DeepWiki](https://deepwiki.com/Arean-Max/synchro-nova)
- 🗺️ [Sơ đồ kiến ​​trúc trên GitDiagram](https://gitdiagram.com/arean-max/synchro-nova)
- 📋 [Hoàn thành các chỉnh sửa & tham khảo sổ đăng ký (TWEAKS_REFERENCE.vi.md)](docs/TWEAKS_REFERENCE.vi.md)

---

### Xây dựng từ nguồn (dành cho nhà phát triển)

```bash
git clone https://github.com/Arean-Max/synchro-nova.git
cd synchro-nova
npm install
npm run dev       #chế độ phát triển
npm run build     # bản phát hành
```

---

### Người đóng góp

- [sotoykor-source](https://github.com/sotoykor-source) — Người sáng tạo ban đầu và tác giả ngược dòng
- [Arean-Max](https://github.com/Arean-Max) — Phát triển và bảo trì

---

### Giấy phép

[MIT](LICENSE) — tự do sử dụng, sửa đổi và chia sẻ.
