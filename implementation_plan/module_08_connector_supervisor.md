# Module 08 - Connector Supervisor

## Trách nhiệm

Quản lý vòng đời các connector, health check, failover và kill switch.

## Task chi tiết

- Tạo `ConnectorSupervisor`.
- Start primary connector khi session bắt đầu.
- Theo dõi health metric:
  - connected/disconnected
  - last_event_at
  - retry_count
  - decode_error_count
  - no_event_duration
- Định nghĩa state machine của connector:
  ```text
  idle -> starting -> healthy -> degraded -> retrying -> fallback -> manual_required -> stopped
  ```
- Định nghĩa health event mà sidecar phải emit:
  ```json
  {"type":"health","stage":"connected","ok":true,"ts":"..."}
  {"type":"health","stage":"receiving","ok":true,"ts":"..."}
  {"type":"error","stage":"protobuf_decode","code":"DECODE_FAILED","ts":"..."}
  {"type":"error","stage":"websocket_connect","code":"WS_CLOSED","ts":"..."}
  {"type":"error","stage":"process","code":"EXITED_UNEXPECTEDLY","ts":"..."}
  ```
- Lưu health metric:
  - `decode_error_count_30s`: số lỗi decode protobuf trong 30 giây gần nhất.
  - `retry_count_current_failure`: số lần app đã thử khởi động lại primary connector trong cùng một lỗi hiện tại.
  - `last_event_at`: thời điểm gần nhất nhận được comment/gift/like/system event hợp lệ.
  - `no_event_duration`: thời gian từ `last_event_at` đến hiện tại.
  - `unexpected_exit_count_5m`: số lần sidecar exit bất thường trong 5 phút gần nhất.
- Định nghĩa rule lỗi cụ thể:
  ```text
  Rule A: protobuf_decode_error > 20 lần / 30 giây
  Rule B: primary connector lỗi, retry 3 lần cách nhau 5 giây vẫn không phục hồi
  Rule C: không có event > 90 giây khi live active
  Rule D: sidecar process exit bất thường
  ```
- Giải thích Rule A - `protobuf_decode_error > 20 lần / 30 giây`:
  - Ý nghĩa: sidecar vẫn nhận được frame từ TikTok nhưng không giải mã được payload.
  - Nguyên nhân thường gặp: TikTok đổi protobuf schema, đổi field mapping hoặc thư viện connector bị lỗi decode.
  - Vì sao dùng ngưỡng 20/30s: một vài lỗi decode lẻ tẻ có thể do packet lạ; lỗi liên tục trong 30 giây là dấu hiệu engine không còn tin cậy.
  - Action:
    1. Chuyển WebSocket status sang `degraded`.
    2. Ghi `connector_health_logs` với reason `protobuf_decode_spike`.
    3. Thực hiện retry primary connector tối đa 3 lần, mỗi lần cách nhau 5 giây.
    4. Nếu sau 3 lần retry vẫn vượt ngưỡng, dừng primary connector và chuyển sang Scrapling fallback.
    5. Nếu fallback bị tắt hoặc chạy lỗi, hiển thị Manual Capture Mode.
- Giải thích Rule B - retry 3 lần, mỗi lần cách 5 giây:
  - Ý nghĩa: primary connector gặp lỗi kết nối hoặc rớt WebSocket; app chỉ thử phục hồi ngắn, không reconnect vô hạn.
  - Nguyên nhân thường gặp: TikTok reject connection, mạng user yếu, endpoint/header/signing có vấn đề, hoặc live room không ổn định.
  - Vì sao chọn 3 lần x 5 giây: đủ để vượt qua lỗi mạng thoáng qua, nhưng không làm seller chờ lâu khi đang live.
  - Action:
    1. Chuyển WebSocket status sang `degraded`.
    2. Retry primary connector lần 1 sau 5 giây.
    3. Nếu vẫn lỗi, retry lần 2 sau 5 giây.
    4. Nếu vẫn lỗi, retry lần 3 sau 5 giây.
    5. Nếu retry lần 3 vẫn lỗi, dừng primary connector và chuyển sang Scrapling fallback.
    6. Tổng thời gian chờ tối đa trước khi failover khoảng 15-20 giây.
    7. Nếu user đang dùng Manual Mode, không ép chuyển lại WebSocket.
- Giải thích Rule C - `không có event > 90 giây khi live active`:
  - Ý nghĩa: connector báo đang kết nối nhưng không nhận event nào trong 90 giây.
  - Điều kiện áp dụng: chỉ áp dụng khi session đang `running` và app xác định live đang active.
  - Cách xác định live active trong MVP:
    - Sidecar resolve được room/live status là active, hoặc
    - trước đó đã nhận event trong cùng session, hoặc
    - user bấm "Tôi đang live" trong UI khi cần xác nhận thủ công.
  - Nguyên nhân thường gặp: stream im ắng thật, connector treo, event mapping sai, WebSocket nhận frame nhưng không emit event.
  - Vì sao dùng ngưỡng 90s: giảm false alarm cho live ít tương tác, nhưng vẫn đủ nhanh để cảnh báo khi connector bị treo.
  - Action:
    1. Hiển thị cảnh báo nhẹ: "Chưa nhận comment/event trong 90 giây".
    2. Gửi lệnh ping/health check tới sidecar.
    3. Nếu sidecar không phản hồi trong 10 giây, retry primary connector tối đa 3 lần, mỗi lần cách 5 giây.
    4. Nếu sau 3 lần retry vẫn không có event, chuyển trạng thái `degraded` và gợi ý Manual Capture.
    5. Không tự động bật Scrapling chỉ vì no-event nếu live có thể đang thật sự vắng comment; cần thêm tín hiệu room active hoặc user xác nhận.
- Giải thích Rule D - `sidecar process exit bất thường`:
  - Ý nghĩa: process Node/Python sidecar tự thoát không theo lệnh stop của Rust.
  - Exit bất thường gồm:
    - exit code khác 0
    - process bị kill
    - stdout protocol lỗi nghiêm trọng làm Rust không đọc được
    - sidecar không gửi heartbeat trong 30 giây và process không phản hồi
  - Action:
    1. Ghi exit code, stderr tail và thời điểm exit vào health log.
    2. Retry primary connector tối đa 3 lần, mỗi lần cách 5 giây.
    3. Nếu retry lần 3 vẫn lỗi hoặc sidecar tiếp tục exit bất thường, đánh dấu primary connector `down`.
    4. Chuyển Scrapling fallback nếu được bật.
    5. Nếu Scrapling cũng lỗi hoặc bị tắt, chuyển UI sang Manual Capture Mode.
- Định nghĩa action theo severity:
  ```text
  healthy:
  - Nhận event bình thường.

  degraded:
  - Vẫn nhận event nhưng có lỗi vượt ngưỡng nhẹ.
  - UI hiện cảnh báo nhỏ.
  - Supervisor retry tối đa 3 lần, mỗi lần cách 5 giây.

  fallback:
  - Primary connector không còn tin cậy.
  - Dừng primary connector.
  - Bật Scrapling fallback.

  manual_required:
  - Cả primary và fallback không khả dụng.
  - UI focus vào Manual Capture input.
  - Không chặn seller tiếp tục lên đơn.
  ```
- Pseudocode xử lý health:
  ```text
  on_sidecar_message(message):
    if message.type == "event":
      last_event_at = now()
      reset_no_event_warning_if_needed()
      push_to_event_ingestion(message)

    if message.type == "error" and message.stage == "protobuf_decode":
      record_decode_error(now())
      if decode_error_count_30s > 20:
        mark_degraded("protobuf_decode_spike")
        retry_primary_up_to_3_times(interval=5s)

    if message.type == "error" and message.stage in ["websocket_connect", "room_resolve", "signing"]:
      mark_degraded("primary_connection_error")
      retry_primary_up_to_3_times(interval=5s)
      if still_failing_after_3_retries:
        switch_to_fallback()

  every_5s:
    if session.running and live_active and no_event_duration > 90s:
      ping_sidecar()
      if no_response_after_10s:
        retry_primary_up_to_3_times(interval=5s)
  ```
- Khi lỗi:
  - ghi health log
  - gửi telemetry nếu user bật
  - retry primary connector tối đa 3 lần, mỗi lần cách 5 giây
  - nếu vượt ngưỡng, chuyển Scrapling
  - nếu Scrapling không khả dụng, đề xuất Manual Capture
- Nhận remote config:
  - primary_enabled
  - fallback_enabled
  - force_manual_mode
  - connector_version
- Hiển thị trạng thái engine cho UI:
  - WebSocket: healthy/degraded/down
  - Scrapling: idle/running/error
  - Manual: available/active
- Không tự động bật browser fallback nếu máy yếu hoặc user tắt.
- Cho phép user chuyển Manual Mode bằng một nút.

## Task coding chi tiết

### M08-T01 - Tạo ConnectorSupervisor service

- Mục tiêu: quản lý trạng thái connector.
- File/thư mục dự kiến: `src-tauri/src/connectors/supervisor.rs`.
- Dependency: M05-T02.
- Output: struct `ConnectorSupervisor`.
- Done khi: service khởi tạo được với session_id.

### M08-T02 - Implement connector state machine

- Mục tiêu: quản lý trạng thái `idle/starting/healthy/degraded/retrying/fallback/manual_required/stopped`.
- File/thư mục dự kiến: `src-tauri/src/connectors/state.rs`.
- Dependency: M08-T01.
- Output: enum state và transition guard.
- Done khi: test transition hợp lệ/không hợp lệ pass.

### M08-T03 - Implement health metric store

- Mục tiêu: lưu decode error, retry count, last event.
- File/thư mục dự kiến: `src-tauri/src/connectors/health.rs`.
- Dependency: M08-T01.
- Output: struct `ConnectorHealthMetrics`.
- Done khi: test sliding window decode 30s pass.

### M08-T04 - Implement retry 3 lần x 5 giây

- Mục tiêu: retry ngắn, không reconnect vô hạn.
- File/thư mục dự kiến: `src-tauri/src/connectors/retry.rs`.
- Dependency: M08-T02.
- Output: function retry primary tối đa 3 lần.
- Done khi: fake connector fail 3 lần thì state sang fallback.

### M08-T05 - Implement Rule A protobuf decode spike

- Mục tiêu: phát hiện schema/decode lỗi.
- File/thư mục dự kiến: `src-tauri/src/connectors/rules.rs`.
- Dependency: M08-T03, M08-T04.
- Output: rule `decode_error_count_30s > 20`.
- Done khi: 21 lỗi/30s trigger degraded + retry.

### M08-T06 - Implement Rule B connection error

- Mục tiêu: xử lý lỗi connect/sign/room.
- File/thư mục dự kiến: `src-tauri/src/connectors/rules.rs`.
- Dependency: M08-T04.
- Output: connection error trigger retry 3 lần.
- Done khi: sau 3 fail state sang fallback.

### M08-T07 - Implement Rule C no-event watchdog

- Mục tiêu: phát hiện sidecar treo.
- File/thư mục dự kiến: `src-tauri/src/connectors/watchdog.rs`.
- Dependency: M08-T03, M08-T04.
- Output: timer 5s kiểm tra `no_event_duration`.
- Done khi: no event 90s + ping fail trigger retry.

### M08-T08 - Implement Rule D process exit

- Mục tiêu: xử lý sidecar crash.
- File/thư mục dự kiến: `src-tauri/src/connectors/process_monitor.rs`.
- Dependency: M08-T04.
- Output: unexpected exit trigger retry/fallback.
- Done khi: fake process exit code 1 được log và retry.

### M08-T09 - Implement fallback switch

- Mục tiêu: dừng primary và bật Scrapling.
- File/thư mục dự kiến: `src-tauri/src/connectors/failover.rs`.
- Dependency: M06-T07, M07-T07, M08-T02.
- Output: `switch_to_fallback`.
- Done khi: primary stopped, fallback running.

### M08-T10 - Emit connector status lên UI

- Mục tiêu: UI thấy trạng thái engine.
- File/thư mục dự kiến: `src-tauri/src/connectors/events.rs`.
- Dependency: M08-T02.
- Output: Tauri event `connector:status`.
- Done khi: UI nhận status healthy/degraded/fallback/manual_required.

## Done khi

- App tự chuyển chế độ khi connector lỗi theo rule.
- User luôn thấy trạng thái hiện tại.
- Manual Mode luôn sẵn sàng.
