# Module 03 - Session

## Trách nhiệm

Quản lý phiên live: tạo phiên, mở lại phiên, đóng phiên, lưu cấu hình bán hàng theo phiên.

## Task chi tiết

- Tạo model `LiveSession`.
- Tạo chức năng tạo phiên live:
  - tên phiên
  - TikTok username/link
  - thời gian tạo
  - trạng thái: draft/running/paused/ended
- Cho phép chọn hoặc nhập nhanh danh sách sản phẩm cho phiên.
- Cho phép cấu hình mặc định:
  - selling mode mặc định
  - cú pháp comment mặc định
  - có cho phép cộng dồn sản phẩm theo khách không
- Tạo chức năng bắt đầu phiên.
- Tạo chức năng tạm dừng phiên.
- Tạo chức năng kết thúc phiên.
- Tạo chức năng mở lại phiên cũ để xem đơn/lịch sử.
- Emit session state cho UI.

## Task coding chi tiết

### M03-T01 - Định nghĩa model `LiveSession`

- Mục tiêu: tạo kiểu dữ liệu session dùng chung Rust/UI.
- File/thư mục dự kiến: `src-tauri/src/sessions/models.rs`, `src/types/session.ts`.
- Dependency: M02-T03.
- Output: Rust struct và TypeScript type có field thống nhất.
- Done khi: serialize/deserialize session qua Tauri command thành công.

### M03-T02 - Tạo `SessionRepository.create`

- Mục tiêu: insert phiên live mới.
- File/thư mục dự kiến: `src-tauri/src/sessions/repository.rs`.
- Dependency: M03-T01.
- Output: function `create_session(input) -> LiveSession`.
- Done khi: test tạo session status `draft` pass.

### M03-T03 - Tạo Tauri command `create_session`

- Mục tiêu: UI tạo được phiên live.
- File/thư mục dự kiến: `src-tauri/src/commands/session.rs`.
- Dependency: M03-T02.
- Output: command nhận name, username/link, config mặc định.
- Done khi: UI gọi command và DB có record mới.

### M03-T04 - Tạo `SessionRepository.get/list`

- Mục tiêu: xem lại phiên hiện có.
- File/thư mục dự kiến: `src-tauri/src/sessions/repository.rs`.
- Dependency: M03-T02.
- Output: function `get_session`, `list_sessions`.
- Done khi: list trả session mới nhất trước.

### M03-T05 - Tạo bảng/link `session_products`

- Mục tiêu: snapshot sản phẩm theo phiên.
- File/thư mục dự kiến: migration trong `src-tauri/migrations/`, `src-tauri/src/sessions/products.rs`.
- Dependency: M02-T04, M03-T02.
- Output: link product vào session với tồn đầu phiên.
- Done khi: cùng product dùng được ở nhiều session khác nhau.

### M03-T06 - Tạo command gán sản phẩm vào phiên

- Mục tiêu: Live Setup chọn/import sản phẩm cho phiên.
- File/thư mục dự kiến: `src-tauri/src/commands/session.rs`.
- Dependency: M03-T05, M04-T02.
- Output: command `set_session_products`.
- Done khi: UI lưu được danh sách sản phẩm của phiên.

### M03-T07 - Tạo command `start_session`

- Mục tiêu: chuyển session từ `draft` sang `running`.
- File/thư mục dự kiến: `src-tauri/src/sessions/service.rs`, `src-tauri/src/commands/session.rs`.
- Dependency: M03-T03, M08-T01.
- Output: cập nhật status, emit event `session:started`.
- Done khi: session running và Connector Supervisor nhận `session_id`.

### M03-T08 - Tạo command `end_session`

- Mục tiêu: kết thúc phiên và dừng ingestion.
- File/thư mục dự kiến: `src-tauri/src/sessions/service.rs`.
- Dependency: M03-T07, M08-T01.
- Output: status `ended`, set `ended_at`, dừng connector.
- Done khi: session ended không nhận event mới.

### M03-T09 - Tạo chế độ xem lại session cũ

- Mục tiêu: mở lịch sử phiên để xem đơn/event.
- File/thư mục dự kiến: `src-tauri/src/commands/session.rs`, `src/features/session-setup/`.
- Dependency: M03-T04, M12-T04.
- Output: command `open_session_history`.
- Done khi: UI xem được event/order của session đã ended.

### M03-T10 - Emit session state cho UI

- Mục tiêu: UI cập nhật trạng thái phiên realtime.
- File/thư mục dự kiến: `src-tauri/src/sessions/events.rs`.
- Dependency: M03-T07, M03-T08.
- Output: Tauri events `session:created`, `session:started`, `session:ended`.
- Done khi: Live Console đổi UI theo event không cần reload.

## Done khi

- User tạo được phiên live.
- Session state lưu local.
- Các module khác nhận được `session_id` để xử lý.
