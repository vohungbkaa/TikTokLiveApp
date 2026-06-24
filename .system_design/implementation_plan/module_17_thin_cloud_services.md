# Module 17 - Thin Cloud Services

## Trách nhiệm

Chỉ phục vụ những phần cần online: license, config, update, kill switch, log lỗi. Không đặt core order workflow lên cloud trong MVP.

Các tính năng cộng đồng như chia sẻ cảnh báo khách bom hàng/nghi phá live chỉ được xem là phase sau MVP, vì cần xử lý quyền riêng tư, chống lạm dụng, kiểm duyệt dữ liệu và cơ chế khiếu nại.

## Service cần có

- License Service:
  - kích hoạt máy
  - kiểm tra hạn dùng
  - giới hạn 1 license dùng tối đa 2 thiết bị trong MVP
- Remote Config Service:
  - bật/tắt primary connector
  - bật/tắt fallback
  - cấu hình ngưỡng failover
  - thông báo version connector
- Update Service:
  - manifest cho Tauri updater
  - version app
  - version sidecar
- Error Log Service:
  - nhận health/error logs
  - dashboard lỗi diện rộng
- Community Risk Alert Service (phase sau MVP):
  - nhận báo cáo khách rủi ro từ shop đã opt-in
  - lưu bằng chứng tối thiểu, lý do, thời điểm, nguồn báo cáo
  - ẩn/giảm thông tin nhạy cảm trước khi chia sẻ cho shop khác
  - tính điểm tin cậy theo số lần báo cáo, độ tin cậy shop báo cáo, trạng thái xác minh
  - trả cảnh báo/risk score cho app khi khách có dấu hiệu trùng khớp
  - hỗ trợ phản hồi, khiếu nại và gỡ cảnh báo sai

## Done khi

- App vẫn dùng được nếu cloud tạm lỗi, trừ tính năng license/update.
- Bật được kill switch khi connector chết diện rộng.
- Grace period khi không gọi được license server là 7 ngày.
- Nếu Community Risk Alert Service lỗi, app vẫn xử lý đơn bình thường và chỉ mất cảnh báo cộng đồng.

## Task coding chi tiết

### M17-T01 - Tạo CloudClient Rust

- Mục tiêu: một HTTP client dùng chung cho cloud services.
- File/thư mục dự kiến: `src-tauri/src/cloud/client.rs`.
- Dependency: M01-T06.
- Output: client có base_url, timeout 5s.
- Done khi: gọi mock endpoint thành công.

### M17-T02 - Tạo Remote Config client

- Mục tiêu: app lấy config connector/fallback.
- File/thư mục dự kiến: `src-tauri/src/cloud/remote_config.rs`.
- Dependency: M17-T01.
- Output: fetch `primary_enabled`, `fallback_enabled`, `force_manual_mode`, version.
- Done khi: mock config áp dụng vào Connector Supervisor.

### M17-T03 - Tạo License client

- Mục tiêu: kiểm tra quyền dùng app.
- File/thư mục dự kiến: `src-tauri/src/cloud/license.rs`.
- Dependency: M17-T01.
- Output: activate/check license, cache local.
- Done khi: offline vẫn chạy trong grace period 7 ngày.

### M17-T04 - Tạo Update Manifest client

- Mục tiêu: đọc manifest update app/sidecar.
- File/thư mục dự kiến: `src-tauri/src/cloud/update_manifest.rs`.
- Dependency: M17-T01.
- Output: fetch version app/sidecar.
- Done khi: mock manifest trả version mới.

### M17-T05 - Tạo Error Log client

- Mục tiêu: gửi health/error logs lên server.
- File/thư mục dự kiến: `src-tauri/src/cloud/error_log.rs`.
- Dependency: M17-T01, M16-T06.
- Output: send sanitized diagnostic payload.
- Done khi: mock server nhận payload.

### M17-T06 - Tạo local cache cho cloud config

- Mục tiêu: app vẫn dùng khi cloud lỗi.
- File/thư mục dự kiến: `src-tauri/src/cloud/cache.rs`.
- Dependency: M02-T06, M17-T02.
- Output: lưu config/license check gần nhất.
- Done khi: mất mạng dùng config cache.

### M17-T07 - Tạo kill switch integration

- Mục tiêu: server tắt primary connector diện rộng.
- File/thư mục dự kiến: `src-tauri/src/cloud/kill_switch.rs`.
- Dependency: M17-T02, M08-T09.
- Output: config `primary_enabled=false` chuyển fallback/manual.
- Done khi: mock config kill switch làm primary không start.

### M17-T08 - Thiết kế Community Risk Alert Service (phase sau MVP)

- Mục tiêu: xác định thiết kế trước khi triển khai cảnh báo khách rủi ro dùng chung giữa các shop.
- File/thư mục dự kiến: `.system_design/community_risk_alert.md`, cloud service spec.
- Dependency: M13-T07.
- Output: spec về dữ liệu được chia sẻ, consent/opt-in, risk score, bằng chứng, khiếu nại, retention, rate limit và anti-abuse.
- Done khi: có tài liệu đủ để review pháp lý/sản phẩm trước khi coding.
