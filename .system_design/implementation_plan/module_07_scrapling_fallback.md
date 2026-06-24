# Module 07 - Scrapling Fallback Sidecar

## Trách nhiệm

Đọc DOM comment TikTok web khi WebSocket connector lỗi. Đây là chế độ cứu live, không phải engine chính.

## Task chi tiết

- Tạo project Python trong `sidecars/scrapling-fallback`.
- Dùng Python 3.11.
- Dùng `uv` để quản lý dependency Python.
- Cài `scrapling`, `playwright` và `pyinstaller`.
- Cài Chromium runtime bằng `playwright install chromium`.
- Tạo CLI:
  ```text
  scrapling-fallback --live-url <url> --session-id <session_id>
  ```
- Tạo output protocol giống Node sidecar.
- Mở TikTok live web.
- Xác định DOM selector cho comment.
- Dùng DOM polling interval 500ms để lấy comment mới trong MVP.
- Normalize comment tối thiểu:
  - display name
  - comment text
  - timestamp received
- Dedupe comment DOM.
- Giới hạn CPU/RAM:
  - polling interval 500ms
  - timeout
  - max browser lifetime theo phiên fallback
- Emit health:
  - browser_starting
  - page_loaded
  - reading_dom
  - selector_missing
- Đóng gói bằng PyInstaller.
- Tạo script `uv run pyinstaller scrapling_fallback.spec`.
- Tích hợp với Tauri sidecar.

## Task coding chi tiết

### M07-T01 - Khởi tạo Python fallback project

- Mục tiêu: tạo project Python sidecar.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/pyproject.toml`, `src/main.py`.
- Dependency: M01-T01.
- Output: `uv run` chạy CLI rỗng.
- Done khi: CLI emit health JSON Lines mẫu.

### M07-T02 - Cài Scrapling/Playwright/PyInstaller

- Mục tiêu: chuẩn bị dependency fallback.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/pyproject.toml`.
- Dependency: M07-T01.
- Output: dependencies locked.
- Done khi: `uv run playwright install chromium` chạy thành công.

### M07-T03 - Implement JSON Lines protocol

- Mục tiêu: protocol giống Node sidecar.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/src/protocol.py`.
- Dependency: M07-T01.
- Output: `emit_health`, `emit_event`, `emit_error`.
- Done khi: stdout line parse JSON được.

### M07-T04 - Mở TikTok live page

- Mục tiêu: load live web trong Chromium.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/src/browser.py`.
- Dependency: M07-T02, M07-T03.
- Output: page loaded hoặc error `page_load`.
- Done khi: URL test emit `page_loaded`.

### M07-T05 - DOM polling comment

- Mục tiêu: lấy comment từ DOM mỗi 500ms.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/src/comment_reader.py`.
- Dependency: M07-T04.
- Output: comment text/display name/timestamp.
- Done khi: fake DOM test đọc được comment mới.

### M07-T06 - Dedupe DOM comment

- Mục tiêu: tránh emit trùng do polling.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/src/dedupe.py`.
- Dependency: M07-T05.
- Output: cache hash comment trong time window.
- Done khi: cùng comment không emit 2 lần liên tiếp.

### M07-T07 - Rust wrapper cho Python sidecar

- Mục tiêu: Supervisor bật/tắt fallback.
- File/thư mục dự kiến: `src-tauri/src/connectors/scrapling_sidecar.rs`.
- Dependency: M07-T03, M05-T01.
- Output: spawn process, parse JSON Lines.
- Done khi: fake Python sidecar đẩy event vào ingestion.

### M07-T08 - Build bằng PyInstaller

- Mục tiêu: đóng gói fallback.
- File/thư mục dự kiến: `sidecars/scrapling-fallback/scrapling_fallback.spec`.
- Dependency: M07-T06.
- Output: binary fallback.
- Done khi: binary chạy CLI và emit health.

## Done khi

- App bật được Scrapling fallback khi cần.
- Comment DOM được đưa về pipeline.
- UI hiển thị rõ đây là chế độ dự phòng, có thể sót comment khi live đông.
- stdout của sidecar chỉ chứa JSON Lines hợp lệ; log kỹ thuật ghi stderr.
