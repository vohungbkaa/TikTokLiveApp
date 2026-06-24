# Module 11 - Selling Mode Engine

## Trách nhiệm

Áp dụng rule bán hàng theo từng mode: Stock, Unique, Preorder.

## Task chi tiết

- Tạo interface:
  ```text
  apply(parsed_intent, live_event, session_context) -> OrderClaimResult
  ```
- Implement Stock Mode:
  - check SKU/variant tồn tại
  - check tồn kho
  - trừ tồn/reserve tồn trong transaction
  - nếu hết hàng, tạo claim `out_of_stock`
- Implement Unique Mode:
  - check claim đầu tiên theo SKU/variant
  - nếu chưa có winner, tạo claim `won`
  - nếu đã có winner, tạo claim `lost` hoặc `waitlist`
- Implement Preorder Mode:
  - nhận đơn không cần tồn
  - nếu có quota thì check quota
- Xử lý `pending_info`:
  - thiếu size/màu
  - SKU có nhiều variant nhưng comment không nói rõ
- Xử lý `needs_review`:
  - parser confidence thấp
  - nhiều SKU trong một comment
  - khách comment mơ hồ
- Dedupe business:
  - cùng khách, cùng SKU, comment lặp trong vài giây
  - cho phép cấu hình cộng dồn hay bỏ qua
- Viết test cho race condition:
  - 2 comment cùng SKU gần như cùng lúc
  - Unique Mode chỉ có 1 winner
  - Stock Mode không trừ quá tồn

## Task coding chi tiết

### M11-T01 - Tạo OrderClaimResult model

- Mục tiêu: chuẩn hóa output engine.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/models.rs`.
- Dependency: M10-T01.
- Output: enum status và result struct.
- Done khi: serialize result sang order module được.

### M11-T02 - Tạo SellingModeEngine interface

- Mục tiêu: một entrypoint áp rule bán hàng.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/engine.rs`.
- Dependency: M11-T01.
- Output: function `apply(parsed_intent, event, context)`.
- Done khi: mock mode compile và trả result.

### M11-T03 - Implement Stock Mode

- Mục tiêu: trừ tồn theo SKU/variant.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/stock.rs`.
- Dependency: M04-T04, M11-T02.
- Output: claim `confirmed` hoặc `out_of_stock`.
- Done khi: test không trừ tồn âm.

### M11-T04 - Implement Unique Mode

- Mục tiêu: chỉ một winner cho SKU/variant.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/unique.rs`.
- Dependency: M11-T02, M02-T07.
- Output: claim `won`, comment sau `lost/waitlist`.
- Done khi: 2 comment cùng SKU chỉ có 1 winner.

### M11-T05 - Implement Preorder Mode

- Mục tiêu: nhận đơn không cần tồn chặt.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/preorder.rs`.
- Dependency: M11-T02.
- Output: claim `confirmed` theo quota.
- Done khi: quota 10 thì claim thứ 11 vào waitlist.

### M11-T06 - Implement pending_info rule

- Mục tiêu: không xử lý sai khi thiếu size/màu.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/validation.rs`.
- Dependency: M10-T06, M11-T02.
- Output: claim `pending_info`.
- Done khi: product có variant bắt buộc nhưng comment thiếu variant -> pending.

### M11-T07 - Implement business dedupe

- Mục tiêu: xử lý comment lặp của cùng khách.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/dedupe.rs`.
- Dependency: M11-T02.
- Output: dedupe theo user/SKU/time window.
- Done khi: cùng khách spam cùng SKU trong 3s không tạo nhiều claim nếu config không cộng dồn.

### M11-T08 - Transaction test cho race condition

- Mục tiêu: đảm bảo concurrency an toàn.
- File/thư mục dự kiến: `src-tauri/src/selling_modes/tests.rs`.
- Dependency: M11-T03, M11-T04.
- Output: tests concurrent claims.
- Done khi: SQLite transaction giữ invariant tồn kho/winner.

## Done khi

- Mỗi parsed comment tạo claim đúng theo mode.
- Không trừ tồn âm.
- Unique Mode không có 2 winner cho cùng SKU/variant.
