# Module 12 - Order

## Trách nhiệm

Gom claim thành order draft theo khách, quản lý trạng thái đơn và xuất/in.

## Task chi tiết

- Tạo order draft khi claim hợp lệ.
- Gom item theo customer:
  - cùng session
  - cùng user_id/unique_id nếu có
  - fallback theo display_name/manual name
- Cho phép cộng item nếu khách chốt nhiều mã.
- Cho phép sửa order draft:
  - sửa số lượng
  - đổi variant
  - xóa item
  - thêm ghi chú
- Quản lý trạng thái:
  - draft
  - pending_info
  - confirmed
  - printed
  - exported
  - cancelled
- Tính tổng tiền.
- Ghi audit khi sửa đơn.
- UI action:
  - xác nhận đơn
  - hủy item
  - chuyển từ waitlist sang confirmed
  - đánh dấu đã liên hệ khách
- Tạo order summary sau live.

## Task coding chi tiết

### M12-T01 - Định nghĩa Order/OrderItem model

- Mục tiêu: chuẩn hóa đơn nháp.
- File/thư mục dự kiến: `src-tauri/src/orders/models.rs`, `src/types/order.ts`.
- Dependency: M02-T05.
- Output: struct/type `Order`, `OrderItem`.
- Done khi: serialize qua Tauri command.

### M12-T02 - Tạo OrderRepository

- Mục tiêu: CRUD order/order item.
- File/thư mục dự kiến: `src-tauri/src/orders/repository.rs`.
- Dependency: M12-T01.
- Output: create/update/list order.
- Done khi: test tạo order + item pass.

### M12-T03 - Tạo OrderDraftService

- Mục tiêu: gom claim vào đơn theo khách.
- File/thư mục dự kiến: `src-tauri/src/orders/draft_service.rs`.
- Dependency: M11-T01, M12-T02, M13-T01.
- Output: `apply_claim_to_order`.
- Done khi: cùng khách chốt 2 SKU tạo 1 order có 2 item.

### M12-T04 - Tạo command list order by session

- Mục tiêu: UI hiển thị order draft.
- File/thư mục dự kiến: `src-tauri/src/commands/order.rs`.
- Dependency: M12-T02.
- Output: command `list_orders_by_session`.
- Done khi: UI lấy danh sách đơn theo session.

### M12-T05 - Tạo command sửa order item

- Mục tiêu: nhân viên sửa đơn.
- File/thư mục dự kiến: `src-tauri/src/commands/order.rs`.
- Dependency: M12-T02, M04-T04.
- Output: update qty/variant/note.
- Done khi: sửa qty cập nhật tổng tiền và tồn đúng.

### M12-T06 - Tạo order status workflow

- Mục tiêu: quản lý trạng thái đơn.
- File/thư mục dự kiến: `src-tauri/src/orders/status.rs`.
- Dependency: M12-T02.
- Output: transition `draft -> confirmed -> printed/exported`.
- Done khi: transition không hợp lệ bị reject.

### M12-T07 - Ghi audit khi sửa đơn

- Mục tiêu: xem lại ai/sửa gì.
- File/thư mục dự kiến: migration audit, `src-tauri/src/orders/audit.rs`.
- Dependency: M12-T05.
- Output: bảng/order audit records.
- Done khi: sửa item tạo audit log.

### M12-T08 - Tạo order summary cuối phiên

- Mục tiêu: tổng hợp sau live.
- File/thư mục dự kiến: `src-tauri/src/orders/summary.rs`.
- Dependency: M12-T02.
- Output: tổng số đơn, doanh thu, pending, out_of_stock.
- Done khi: command trả summary đúng theo fixture.

## Done khi

- Comment hợp lệ tự gom vào order draft.
- Nhân viên sửa/xác nhận đơn được.
- Có audit cơ bản cho thao tác sửa.
