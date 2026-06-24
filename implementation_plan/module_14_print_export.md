# Module 14 - Print & Export

## Trách nhiệm

Xuất dữ liệu đơn sau live và hỗ trợ in đơn cơ bản.

## Task chi tiết

- Export CSV:
  - mã đơn
  - khách
  - SKU
  - tên sản phẩm
  - variant
  - số lượng
  - giá
  - tổng tiền
  - ghi chú
- Export Excel bằng crate `umya-spreadsheet`.
- Tạo template in đơn đơn giản:
  - tên shop
  - khách
  - sản phẩm
  - tổng tiền
  - ghi chú
- Tích hợp print dialog hệ điều hành ở P1.
- Đưa tối ưu máy in nhiệt trực tiếp vào Phase 3:
  - khổ giấy
  - template label
  - printer mapping
- Phase 3 dùng command in hệ điều hành trước, chưa tích hợp ESC/POS native cho đến khi có mẫu máy in cụ thể.
- Lưu trạng thái `printed/exported`.

## Task coding chi tiết

### M14-T01 - Tạo ExportOrder DTO

- Mục tiêu: chuẩn hóa dữ liệu xuất.
- File/thư mục dự kiến: `src-tauri/src/export/models.rs`.
- Dependency: M12-T02.
- Output: DTO flatten order/item/customer.
- Done khi: DTO tạo từ order fixture đúng.

### M14-T02 - Implement CSV export

- Mục tiêu: xuất đơn sau live.
- File/thư mục dự kiến: `src-tauri/src/export/csv.rs`.
- Dependency: M14-T01.
- Output: file CSV UTF-8.
- Done khi: file mở được bằng Excel/Google Sheets và đủ cột.

### M14-T03 - Implement Excel export

- Mục tiêu: xuất `.xlsx`.
- File/thư mục dự kiến: `src-tauri/src/export/excel.rs`.
- Dependency: M14-T01.
- Output: file `.xlsx` bằng `umya-spreadsheet`.
- Done khi: file mở được và có sheet Orders.

### M14-T04 - Tạo command export session orders

- Mục tiêu: UI bấm xuất file.
- File/thư mục dự kiến: `src-tauri/src/commands/export.rs`.
- Dependency: M14-T02, M14-T03.
- Output: command chọn format csv/xlsx.
- Done khi: export tạo file ở path user chọn.

### M14-T05 - Tạo print template HTML

- Mục tiêu: in đơn cơ bản.
- File/thư mục dự kiến: `src-tauri/src/export/print_template.rs`.
- Dependency: M14-T01.
- Output: HTML string cho order.
- Done khi: render có tên shop, khách, items, tổng tiền.

### M14-T06 - Tích hợp print dialog

- Mục tiêu: in qua hệ điều hành.
- File/thư mục dự kiến: `src-tauri/src/commands/print.rs`.
- Dependency: M14-T05.
- Output: command print order.
- Done khi: mở được print dialog hoặc tạo print preview theo OS.

### M14-T07 - Cập nhật trạng thái printed/exported

- Mục tiêu: biết đơn đã xử lý.
- File/thư mục dự kiến: `src-tauri/src/export/status.rs`.
- Dependency: M12-T06, M14-T04.
- Output: set order status after print/export.
- Done khi: export xong order có status `exported`.

## Done khi

- Sau live xuất CSV/Excel được.
- In đơn cơ bản được.
- Đơn đã in/xuất có trạng thái rõ.
