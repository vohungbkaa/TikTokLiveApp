# Module 09 - Manual Capture

## Trách nhiệm

Cho phép nhân viên nhập comment thủ công siêu nhanh và đưa vào pipeline như event thật.

## Task chi tiết

- Tạo ô nhập nhanh trong Live Console.
- Hỗ trợ format:
  ```text
  @username A12
  linh A12 2 cái
  mai kem01
  chốt A12 đỏ L
  ```
- Parse tách phần khách và phần comment.
- Nếu không có khách, cho phép tạo khách tạm:
  - `Khách lẻ 001`
  - `manual_user_001`
- Autocomplete SKU khi gõ.
- Gợi ý khách đã xuất hiện trong live.
- Gửi event với `source = manual`.
- Gán `sequence_no` theo thời điểm nhập.
- Phím tắt:
  - Enter: gửi
  - Cmd/Ctrl + Enter: gửi và giữ khách hiện tại
  - Esc: clear input
  - mũi tên lên/xuống: chọn gợi ý
- Hiển thị cảnh báo nhỏ:
  - "Manual Mode: thứ tự chốt dựa trên thứ tự nhập."
- Ghi audit trail.

## Task coding chi tiết

### M09-T01 - Tạo Manual Capture UI input

- Mục tiêu: nhập comment thủ công nhanh.
- File/thư mục dự kiến: `src/features/live-console/components/ManualCaptureInput.vue`.
- Dependency: M15-T01.
- Output: input có focus mặc định và submit bằng Enter.
- Done khi: nhập text và emit event frontend.

### M09-T02 - Tạo parser tách khách/comment

- Mục tiêu: hiểu format manual như `@username A12`.
- File/thư mục dự kiến: `src-tauri/src/events/manual_parser.rs`.
- Dependency: M05-T01.
- Output: `{ manual_customer_name, comment_text }`.
- Done khi: test 4 format manual pass.

### M09-T03 - Tạo Tauri command `submit_manual_event`

- Mục tiêu: UI gửi manual event vào pipeline.
- File/thư mục dự kiến: `src-tauri/src/commands/manual.rs`.
- Dependency: M09-T02, M05-T03.
- Output: command tạo `LiveEvent` source `manual`.
- Done khi: manual event lưu DB và lên comment feed.

### M09-T04 - Autocomplete SKU

- Mục tiêu: gợi ý mã hàng khi gõ.
- File/thư mục dự kiến: `src/features/live-console/components/SkuAutocomplete.vue`.
- Dependency: M04-T05, M09-T01.
- Output: dropdown SKU theo session products.
- Done khi: gõ `A` hiện SKU bắt đầu bằng A.

### M09-T05 - Gợi ý khách trong phiên

- Mục tiêu: nhập nhanh tên khách cũ.
- File/thư mục dự kiến: `src/features/live-console/components/CustomerAutocomplete.vue`.
- Dependency: M13-T03, M09-T01.
- Output: dropdown khách đã xuất hiện.
- Done khi: gõ tên hiện khách trong session.

### M09-T06 - Thêm keyboard shortcuts

- Mục tiêu: vận hành không rời bàn phím.
- File/thư mục dự kiến: `src/features/live-console/composables/useManualShortcuts.ts`.
- Dependency: M09-T01.
- Output: Enter submit, Esc clear, Ctrl/Cmd+Enter giữ khách.
- Done khi: test UI shortcut hoạt động.

### M09-T07 - Ghi audit manual event

- Mục tiêu: phân biệt event nhập tay và tự động.
- File/thư mục dự kiến: `src-tauri/src/events/audit.rs`.
- Dependency: M09-T03.
- Output: audit record cho event source manual.
- Done khi: xem event history thấy source `manual`.

## Done khi

- Nhân viên có thể nhập tay liên tục mà không rời bàn phím.
- Event manual đi qua cùng parser/order pipeline.
- Có dấu vết phân biệt event tự động và manual.
