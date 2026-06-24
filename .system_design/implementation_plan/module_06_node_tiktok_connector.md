# Module 06 - Node TikTok Connector Sidecar

## Trách nhiệm

Bọc `TikTok-Live-Connector` thành một process riêng, xuất event/health/error theo protocol do app kiểm soát.

## Task chi tiết

- Tạo project Node trong `sidecars/tiktok-connector`.
- Dùng Node.js LTS và `pnpm`.
- Cài `TikTok-Live-Connector`.
- Tạo CLI interface:
  ```text
  tiktok-connector --username <username> --session-id <session_id>
  ```
- Giao tiếp với Rust qua stdout JSON Lines.
- Nhận lệnh stop từ Rust qua stdin JSON Lines.
- Ghi log kỹ thuật vào stderr, không ghi log thường vào stdout.
- Định nghĩa message protocol:
  ```json
  {"type":"health","stage":"connecting","ok":true}
  {"type":"event","event_type":"comment","data":{}}
  {"type":"error","stage":"protobuf_decode","code":"DECODE_FAILED","message":"..."}
  ```
- Implement connect by username/link.
- Emit comment event.
- Emit connection state:
  - resolving_room
  - connecting
  - connected
  - receiving
  - disconnected
  - reconnecting
- Emit error stage:
  - room_resolve
  - signing
  - websocket_connect
  - protobuf_decode
  - event_mapping
  - unknown
- Implement auto reconnect có giới hạn.
- Implement graceful shutdown.
- Đóng gói sidecar để Tauri gọi được bằng `pkg`.
- Tạo script `pnpm build:sidecar` để build binary cho macOS, Windows và Linux.
- Tạo test script chạy connector độc lập.

## Task coding chi tiết

### M06-T01 - Khởi tạo Node sidecar project

- Mục tiêu: tạo package riêng cho connector.
- File/thư mục dự kiến: `sidecars/tiktok-connector/package.json`, `src/index.ts`.
- Dependency: M01-T01.
- Output: `pnpm dev` chạy CLI rỗng.
- Done khi: CLI in health JSON Lines mẫu.

### M06-T02 - Cài và bọc TikTok-Live-Connector

- Mục tiêu: kết nối live bằng thư viện chính.
- File/thư mục dự kiến: `sidecars/tiktok-connector/src/client.ts`.
- Dependency: M06-T01.
- Output: wrapper nhận username/live url.
- Done khi: chạy CLI với username test connect được hoặc trả lỗi stage rõ.

### M06-T03 - Implement JSON Lines stdout protocol

- Mục tiêu: stdout chỉ chứa message máy đọc được.
- File/thư mục dự kiến: `sidecars/tiktok-connector/src/protocol.ts`.
- Dependency: M06-T01.
- Output: `emitHealth`, `emitEvent`, `emitError`.
- Done khi: mỗi stdout line parse JSON được.

### M06-T04 - Emit comment event chuẩn

- Mục tiêu: chuyển event TikTok thành payload chuẩn.
- File/thư mục dự kiến: `sidecars/tiktok-connector/src/mappers.ts`.
- Dependency: M06-T02, M06-T03.
- Output: comment có `user_id`, `unique_id`, `display_name`, `comment`.
- Done khi: Rust mock parser đọc được output.

### M06-T05 - Emit health/error stage

- Mục tiêu: Supervisor biết lỗi nằm ở đâu.
- File/thư mục dự kiến: `sidecars/tiktok-connector/src/health.ts`.
- Dependency: M06-T03.
- Output: stages `room_resolve`, `websocket_connect`, `protobuf_decode`, `event_mapping`.
- Done khi: lỗi giả lập emit đúng stage.

### M06-T06 - Nhận lệnh stop qua stdin

- Mục tiêu: Rust dừng sidecar sạch.
- File/thư mục dự kiến: `sidecars/tiktok-connector/src/stdin.ts`.
- Dependency: M06-T03.
- Output: nhận `{"type":"command","command":"stop"}`.
- Done khi: process exit code 0 khi nhận stop.

### M06-T07 - Tạo Rust SidecarProcess wrapper

- Mục tiêu: Rust start/stop/đọc stdout sidecar.
- File/thư mục dự kiến: `src-tauri/src/connectors/node_sidecar.rs`.
- Dependency: M06-T03, M05-T01.
- Output: wrapper spawn process và parse JSON Lines.
- Done khi: test với fake sidecar nhận event.

### M06-T08 - Build sidecar bằng pkg

- Mục tiêu: tạo binary phân phối cùng Tauri.
- File/thư mục dự kiến: `sidecars/tiktok-connector/package.json`, `scripts/build-sidecars/`.
- Dependency: M06-T06.
- Output: binary macOS/Windows/Linux.
- Done khi: Tauri dev gọi được binary build.

## Done khi

- Rust start/stop được Node sidecar.
- Comment từ TikTok được emit về app dưới dạng JSON chuẩn.
- Khi lỗi, sidecar báo đúng stage để supervisor xử lý.
- stdout của sidecar chỉ chứa JSON Lines hợp lệ.
