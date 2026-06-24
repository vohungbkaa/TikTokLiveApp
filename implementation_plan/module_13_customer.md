# Module 13 - Customer

## Trách nhiệm

Quản lý thông tin khách trong phiên live và lịch sử cơ bản.

## Task chi tiết

- Tạo customer identity resolver:
  - ưu tiên `user_id`
  - sau đó `unique_id`
  - sau đó manual name/display_name
- Tạo customer record khi có comment mua hàng.
- Lưu:
  - display name
  - unique id
  - phone
  - address
  - note
  - tags
- Hỗ trợ nhập/sửa thông tin khách.
- Hỗ trợ tag:
  - khách quen
  - cần xác minh
  - boom nội bộ
- Hỗ trợ blacklist nội bộ của shop ở Phase 3.
- Gợi ý khách trong Manual Capture.

## Task coding chi tiết

### M13-T01 - Định nghĩa Customer model

- Mục tiêu: chuẩn hóa thông tin khách.
- File/thư mục dự kiến: `src-tauri/src/customers/models.rs`, `src/types/customer.ts`.
- Dependency: M02-T05.
- Output: struct/type Customer.
- Done khi: serialize qua Tauri command.

### M13-T02 - Tạo CustomerRepository

- Mục tiêu: CRUD khách hàng.
- File/thư mục dự kiến: `src-tauri/src/customers/repository.rs`.
- Dependency: M13-T01.
- Output: create/update/find.
- Done khi: test create/find pass.

### M13-T03 - Tạo CustomerIdentityResolver

- Mục tiêu: gom đơn đúng khách.
- File/thư mục dự kiến: `src-tauri/src/customers/identity.rs`.
- Dependency: M13-T02, M05-T01.
- Output: resolve theo `user_id`, `unique_id`, display/manual name.
- Done khi: cùng user_id trả cùng customer.

### M13-T04 - Tạo customer từ LiveEvent

- Mục tiêu: auto tạo khách khi có comment mua.
- File/thư mục dự kiến: `src-tauri/src/customers/service.rs`.
- Dependency: M13-T03.
- Output: `get_or_create_from_event`.
- Done khi: event mới tạo customer record.

### M13-T05 - Tạo command sửa thông tin khách

- Mục tiêu: nhập phone/address/note.
- File/thư mục dự kiến: `src-tauri/src/commands/customer.rs`.
- Dependency: M13-T02.
- Output: command update customer profile.
- Done khi: UI sửa và DB cập nhật.

### M13-T06 - Tạo customer tags

- Mục tiêu: đánh dấu khách quen/cần xác minh/boom nội bộ.
- File/thư mục dự kiến: migration tags, `src-tauri/src/customers/tags.rs`.
- Dependency: M13-T02.
- Output: add/remove/list tags.
- Done khi: gắn tag và query theo tag.

### M13-T07 - API gợi ý khách cho Manual Capture

- Mục tiêu: autocomplete khách.
- File/thư mục dự kiến: `src-tauri/src/commands/customer.rs`.
- Dependency: M13-T03.
- Output: command `search_customers_in_session`.
- Done khi: search theo prefix trả khách trong phiên.

## Done khi

- Đơn được gom đúng theo khách trong khả năng dữ liệu có được.
- Manual Mode gợi ý được khách.
