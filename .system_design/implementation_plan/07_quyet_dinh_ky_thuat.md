# 07 - Quyết định kỹ thuật đã chốt

File này chốt các lựa chọn kỹ thuật để task triển khai không còn ở dạng "cân nhắc".

## App desktop

- Desktop framework: `Tauri 2`.
- Frontend build tool: `Vite`.
- Frontend framework: `Vue 3`.
- Frontend language: `TypeScript`.
- State management: `Pinia`.
- Router: `Vue Router`.
- UI component nền: tự xây component nội bộ bằng CSS/Tailwind utility, không dùng framework UI nặng trong MVP.
- Icon: `lucide-vue-next`.
- Virtual list: `vue-virtual-scroller`.

## Rust core

- Async runtime: `tokio`.
- Database: `SQLite`.
- Rust DB library: `sqlx` với feature `sqlite`, `runtime-tokio`, `macros`, `chrono`, `json`.
- Migration: `sqlx::migrate!`.
- Serialization: `serde`, `serde_json`.
- Time: `chrono`.
- Error handling: `thiserror` cho domain error, `anyhow` chỉ dùng ở boundary/bootstrap.
- Logging: `tracing`, `tracing-subscriber`.
- CSV export: crate `csv`.
- Excel export: `umya-spreadsheet`.

## Sidecar

- Node TikTok connector giao tiếp với Rust bằng stdout JSON Lines.
- Python Scrapling fallback giao tiếp với Rust bằng stdout JSON Lines.
- Rust gửi lệnh stop cho sidecar bằng stdin JSON Lines.
- Không dùng local HTTP/WebSocket giữa Rust và sidecar trong MVP.
- Mỗi dòng stdout phải là một JSON object hợp lệ.
- Sidecar log kỹ thuật không được trộn vào stdout event; log kỹ thuật ghi vào stderr.

## Node TikTok Connector

- Runtime: Node.js LTS.
- Package manager: `pnpm`.
- Library chính: `TikTok-Live-Connector`.
- Build sidecar: bundle bằng `pkg`.
- Dev mode: chạy trực tiếp bằng Node.js thông qua `pnpm dev`.
- Protocol: JSON Lines.

## Python Scrapling Fallback

- Python version: 3.11.
- Library chính: `Scrapling`.
- Browser automation fallback: dùng Chromium do Playwright/Scrapling quản lý.
- DOM strategy MVP: polling interval 500ms, chưa dùng MutationObserver.
- Package sidecar: `PyInstaller`.
- Dependency manager: `uv`.

## Parser

- MVP dùng rule-based parser.
- Không dùng AI parser trong P0/P1.
- Text normalizer luôn chuẩn hóa Unicode, lowercase, trim, bỏ emoji/ký tự điều khiển.
- SKU matching luôn hỗ trợ exact match, case-insensitive, bỏ dấu tiếng Việt và alias.

## Export/In

- P0 xuất CSV bắt buộc.
- P1 xuất Excel bằng `umya-spreadsheet`.
- P1 in qua print dialog hệ điều hành.
- Tích hợp máy in nhiệt trực tiếp để Phase 3.

## Cloud

- MVP core chạy local.
- Cloud chỉ phục vụ license, remote config, update manifest và error log.
- Nếu cloud lỗi, app vẫn cho mở phiên live local trong grace period cố định 7 ngày.
