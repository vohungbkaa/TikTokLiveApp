# Module 16 - Telemetry & Diagnostics

## Trách nhiệm

Phát hiện lỗi connector nhanh, ghi log đủ để debug và hỗ trợ update/fallback.

## Task chi tiết

- Local logs:
  - connector state
  - sidecar start/stop
  - error stage
  - parser error
  - DB error
- Health metrics:
  - last event time
  - comment count/minute
  - reconnect count
  - decode error count
  - fallback activation count
- Log viewer đơn giản trong app.
- Export diagnostic bundle:
  - app version
  - connector version
  - health logs
  - sanitized errors
- Remote error report nếu user bật:
  - không gửi nội dung comment nhạy cảm nếu chưa xin phép
  - ưu tiên hash/sample metadata
- Canary monitor ở server phase sau:
  - kiểm tra connector với live public
  - báo degraded/down

## Task coding chi tiết

### M16-T01 - Tạo DiagnosticLog model

- Mục tiêu: chuẩn hóa log lỗi.
- File/thư mục dự kiến: `src-tauri/src/diagnostics/models.rs`.
- Dependency: M02-T06.
- Output: model app/connector/parser/db log.
- Done khi: serialize log thành JSON.

### M16-T02 - Tạo DiagnosticsRepository

- Mục tiêu: lưu health/error logs.
- File/thư mục dự kiến: `src-tauri/src/diagnostics/repository.rs`.
- Dependency: M16-T01.
- Output: insert/list logs.
- Done khi: ghi và query log theo session.

### M16-T03 - Tích hợp connector health logs

- Mục tiêu: lưu lỗi connector từ Supervisor.
- File/thư mục dự kiến: `src-tauri/src/connectors/health_log.rs`.
- Dependency: M08-T03, M16-T02.
- Output: mỗi rule trigger ghi log.
- Done khi: protobuf spike tạo log reason đúng.

### M16-T04 - Tạo log viewer command

- Mục tiêu: UI xem log cơ bản.
- File/thư mục dự kiến: `src-tauri/src/commands/diagnostics.rs`.
- Dependency: M16-T02.
- Output: command `list_diagnostic_logs`.
- Done khi: Settings/Diagnostics lấy log được.

### M16-T05 - Tạo diagnostic bundle export

- Mục tiêu: user gửi file debug cho support.
- File/thư mục dự kiến: `src-tauri/src/diagnostics/bundle.rs`.
- Dependency: M16-T02.
- Output: `.zip` hoặc folder chứa app version, logs, config sanitized.
- Done khi: bundle không chứa raw comment nếu chưa bật consent.

### M16-T06 - Tạo remote error report client

- Mục tiêu: gửi lỗi khi user bật telemetry.
- File/thư mục dự kiến: `src-tauri/src/diagnostics/remote.rs`.
- Dependency: M16-T05, M17-T02.
- Output: HTTP client gửi sanitized payload.
- Done khi: mock server nhận report.

### M16-T07 - Tạo metrics counters

- Mục tiêu: thống kê comment/retry/fallback.
- File/thư mục dự kiến: `src-tauri/src/diagnostics/metrics.rs`.
- Dependency: M05-T03, M08-T03.
- Output: counters comment/minute, retry_count, fallback_count.
- Done khi: metrics cập nhật theo event mock.

## Done khi

- Khi lỗi connector, dev biết lỗi nằm ở stage nào.
- User gửi được diagnostic bundle.
