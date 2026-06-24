# Module 05 - Event Ingestion

## Trách nhiệm

Nhận event từ các nguồn khác nhau và đẩy vào pipeline theo cùng một interface.

## Nguồn event

- WebSocket connector sidecar.
- Scrapling fallback sidecar.
- Manual Capture Mode.
- Extension point đã chuẩn bị cho OCR và import comment ở phase mở rộng.

## Task chi tiết

- Định nghĩa trait/interface nội bộ:
  ```text
  EventSource
  - start(session_config)
  - stop()
  - health()
  - on_event(callback)
  - on_error(callback)
  ```
- Tạo `EventIngestionService`.
- Tạo event queue trong Rust:
  - nhận event
  - gán `sequence_no`
  - debounce/dedupe cơ bản
  - gửi vào pipeline
- Tạo backpressure strategy:
  - nếu comment quá nhiều, vẫn không làm đứng UI
  - giới hạn batch emit lên frontend
- Tạo dedupe:
  - theo `platform_event_id` nếu có
  - theo `(source, user_id, comment, time_window)` nếu không có event id
- Tạo error handling:
  - source disconnected
  - malformed event
  - queue overflow
- Tạo log event source.

## Task coding chi tiết

### M05-T01 - Định nghĩa model `LiveEvent`

- Mục tiêu: chuẩn hóa event nội bộ.
- File/thư mục dự kiến: `src-tauri/src/events/models.rs`, `src/types/live-event.ts`.
- Dependency: M02-T03.
- Output: Rust/TS type `LiveEvent`.
- Done khi: deserialize event JSON từ sidecar thành công.

### M05-T02 - Tạo EventSource trait

- Mục tiêu: chuẩn interface cho WebSocket/Scrapling/Manual.
- File/thư mục dự kiến: `src-tauri/src/events/source.rs`.
- Dependency: M05-T01.
- Output: trait `EventSource` với `start`, `stop`, `health`.
- Done khi: mock source implement trait compile được.

### M05-T03 - Tạo EventIngestionService

- Mục tiêu: nhận event từ source và đưa vào queue.
- File/thư mục dự kiến: `src-tauri/src/events/ingestion.rs`.
- Dependency: M05-T01, M05-T02.
- Output: service nhận raw event và gán `sequence_no`.
- Done khi: test 3 event có sequence tăng dần.

### M05-T04 - Tạo EventRepository insert

- Mục tiêu: lưu event trước khi xử lý đơn.
- File/thư mục dự kiến: `src-tauri/src/events/repository.rs`.
- Dependency: M05-T01, M02-T03.
- Output: `insert_event`, `list_events_by_session`.
- Done khi: event lưu vào DB và query lại đúng.

### M05-T05 - Tạo dedupe event

- Mục tiêu: tránh xử lý comment trùng.
- File/thư mục dự kiến: `src-tauri/src/events/dedupe.rs`.
- Dependency: M05-T01.
- Output: dedupe theo `platform_event_id` hoặc `(source,user_id,comment,time_window)`.
- Done khi: test comment trùng trong time window bị bỏ qua.

### M05-T06 - Tạo batch emit lên UI

- Mục tiêu: tránh UI giật khi comment nhiều.
- File/thư mục dự kiến: `src-tauri/src/events/ui_emit.rs`.
- Dependency: M05-T03.
- Output: batch event mỗi 100ms hoặc tối đa 50 event/lần.
- Done khi: UI nhận batch thay vì từng event riêng lẻ.

### M05-T07 - Tạo event pipeline hook

- Mục tiêu: nối ingestion -> parser -> selling engine.
- File/thư mục dự kiến: `src-tauri/src/events/pipeline.rs`.
- Dependency: M05-T03, M10-T01, M11-T01.
- Output: function `process_live_event`.
- Done khi: mock event đi qua parser và tạo claim mock.

## Done khi

- Nhiều nguồn event đẩy được vào cùng pipeline.
- Event luôn có `sequence_no`.
- UI không bị phụ thuộc format riêng của connector.
