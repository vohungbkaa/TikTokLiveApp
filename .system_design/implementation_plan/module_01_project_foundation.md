# Module 01 - Project Foundation

## Trách nhiệm

Khởi tạo cấu trúc dự án, tooling, conventions, build scripts và môi trường phát triển.

## Task chi tiết

- Khởi tạo Tauri project với Vue 3 và Rust backend.
- Thiết lập TypeScript cho frontend.
- Thiết lập `Vue Router` cho các màn hình: Live Setup, Live Console, Order Review, Settings.
- Thiết lập `Pinia` làm state management chính.
- Thiết lập `Vite` làm build tool frontend.
- Cài `lucide-vue-next` cho icon.
- Cài `vue-virtual-scroller` cho danh sách comment realtime.
- Thiết lập Rust workspace/module layout.
- Thiết lập formatter/linter:
  - Rust: `rustfmt`, `clippy`.
  - Frontend: ESLint/Prettier.
- Thiết lập cấu trúc thư mục:
  ```text
  src/
  src-tauri/
  sidecars/
  docs/
  scripts/
  ```
- Thiết lập config theo environment:
  - dev
  - staging
  - production
- Thiết lập logging local.
- Thiết lập `tracing` và `tracing-subscriber` trong Rust.
- Thiết lập error boundary cơ bản ở UI.
- Thiết lập CI cơ bản:
  - build frontend
  - cargo check
  - unit test

## Task coding chi tiết

### M01-T01 - Khởi tạo Tauri 2 + Vue 3 + TypeScript

- Mục tiêu: tạo project desktop nền.
- File/thư mục dự kiến: `src/`, `src-tauri/`, `package.json`, `vite.config.ts`.
- Dependency: không có.
- Output: app Tauri mở được màn hình Vue mặc định.
- Done khi: chạy được `pnpm dev` và cửa sổ desktop hiển thị.

### M01-T02 - Cài frontend dependencies nền

- Mục tiêu: cài router, state, icon, virtual list.
- File/thư mục dự kiến: `package.json`, `src/main.ts`.
- Dependency: M01-T01.
- Output: cài `vue-router`, `pinia`, `lucide-vue-next`, `vue-virtual-scroller`.
- Done khi: app boot không lỗi và import thử các package thành công.

### M01-T03 - Thiết lập cấu trúc frontend

- Mục tiêu: tạo layout thư mục feature-based.
- File/thư mục dự kiến: `src/app/`, `src/components/`, `src/features/`, `src/stores/`, `src/types/`.
- Dependency: M01-T01.
- Output: cấu trúc thư mục frontend ban đầu.
- Done khi: `src/main.ts` mount app từ `src/app/App.vue`.

### M01-T04 - Thiết lập cấu trúc Rust core

- Mục tiêu: tạo module layout cho Rust/Tauri.
- File/thư mục dự kiến: `src-tauri/src/commands/`, `db/`, `sessions/`, `events/`, `parser/`, `products/`, `orders/`, `connectors/`.
- Dependency: M01-T01.
- Output: module Rust compile được.
- Done khi: `cargo check` pass.

### M01-T05 - Thiết lập lint/format

- Mục tiêu: thống nhất format code.
- File/thư mục dự kiến: `.prettierrc`, `eslint.config.*`, `rustfmt.toml`.
- Dependency: M01-T01.
- Output: script `lint`, `format`, `cargo fmt`, `cargo clippy`.
- Done khi: chạy lint/format không lỗi trên project mới.

### M01-T06 - Thiết lập logging Rust

- Mục tiêu: log có cấu trúc cho core.
- File/thư mục dự kiến: `src-tauri/src/main.rs`, `src-tauri/src/diagnostics/`.
- Dependency: M01-T04.
- Output: cấu hình `tracing` và `tracing-subscriber`.
- Done khi: app ghi log startup ra console/file local.

### M01-T07 - Thiết lập command health check

- Mục tiêu: kiểm tra IPC Vue <-> Rust.
- File/thư mục dự kiến: `src-tauri/src/commands/app.rs`, `src/features/settings/`.
- Dependency: M01-T04.
- Output: Tauri command `app_health_check`.
- Done khi: UI gọi command và nhận `{ ok: true, version }`.

### M01-T08 - Thiết lập CI cơ bản

- Mục tiêu: đảm bảo project build được tự động.
- File/thư mục dự kiến: `.github/workflows/ci.yml`.
- Dependency: M01-T05.
- Output: workflow chạy install, lint, test, cargo check.
- Done khi: CI pass trên pull request hoặc local act tương đương.

## Done khi

- App Tauri mở được màn hình trống.
- Lệnh build chạy được.
- Rust và frontend có lint/format.
- `Pinia`, `Vue Router`, `lucide-vue-next`, `vue-virtual-scroller` được cài và import thử thành công.
- Có cấu trúc module ban đầu rõ ràng.
